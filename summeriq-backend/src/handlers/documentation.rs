use actix_web::{web, HttpResponse};
use serde::Serialize;
use crate::services::{AIService, StorageService, AnalysisService};
use crate::error::AppError;
use std::fs;
use urlencoding::decode;

#[derive(Serialize)]
pub struct FileAnalysisDoc {
    pub path: String,
    pub name: String,
    pub description: String,
    pub dependencies: Vec<String>,
    pub relationships: Vec<Relationship>,
}

#[derive(Serialize)]
pub struct Relationship {
    pub target_file: String,
    pub relationship_type: String,
    pub description: String,
}

#[derive(Serialize)]
pub struct ProjectDocumentation {
    pub project_name: String,
    pub description: String,
    pub architecture: String,
    pub file_analyses: Vec<FileAnalysisDoc>,
    pub dependencies: Vec<String>,
    pub setup_instructions: String,
}

fn normalize_name(name: &str) -> String {
    name.to_lowercase().replace(|c: char| !c.is_ascii_alphanumeric(), "")
}

pub async fn get_project_documentation(
    path: web::Path<String>,
    ai_service: web::Data<AIService>,
    storage_service: web::Data<StorageService>,
    _analysis_service: web::Data<AnalysisService>,
) -> Result<HttpResponse, AppError> {
    tracing::info!("get_project_documentation: incoming path: {}", path);
    let path = path.into_inner();
    let decoded_path = decode(&path).map(|c| c.to_string()).unwrap_or(path.clone());
    let trimmed = decoded_path.trim_start_matches('/');
    let path_parts: Vec<&str> = trimmed.split('/').collect();
    if path_parts.is_empty() {
        return Err(AppError::BadRequest("Empty path provided".to_string()));
    }
    let project_name = path_parts[0];
    tracing::info!("get_project_documentation: using project_name: {}", project_name);
    let uuid = storage_service.get_file_id(project_name).await?;
    let extracted_dir = format!("extracted_{}", uuid);
    let files = storage_service.list_files(&extracted_dir).await?;

    // Helper to flatten file tree to a list of (path, is_dir)
    fn flatten_files(nodes: &[crate::services::storage::FileNode], parent: &str, out: &mut Vec<(String, bool)>) {
        for node in nodes {
            let full_path = if parent.is_empty() {
                node.name.clone()
            } else {
                format!("{}/{}", parent, node.name)
            };
            out.push((full_path.clone(), node.is_dir));
            if node.is_dir {
                if let Some(children) = &node.children {
                    flatten_files(children, &full_path, out);
                }
            }
        }
    }
    let mut file_list = Vec::new();
    flatten_files(&files, "", &mut file_list);

    // Key files to prioritize for content (entry points, configs, main modules)
    let key_file_names = [
        "main.rs", "App.tsx", "app.tsx", "index.tsx", "index.js", "package.json", "Cargo.toml", "tsconfig.json", "next.config.js", "next.config.mjs", ".env", "README.md"
    ];

    // Step 1: Generate the file/folder structure string
    let mut structure = String::new();
    for (path, is_dir) in &file_list {
        if *is_dir {
            structure.push_str(&format!("[DIR] {}\n", path));
        } else {
            structure.push_str(&format!("      {}\n", path));
        }
    }

    // Step 1: Ask the AI for a high-level architecture summary based on the structure
    let structure_prompt = format!(
        r#"
You are an expert technical writer and software architect. Here is the file and folder structure of a software project:

{structure}

Please give a high-level architectural overview of how the folders and files relate to each other. Focus on helping a junior developer understand how this is structured and why. No file content yet—just structure. Use Markdown formatting, clear sections, and diagrams if helpful.
"#,
        structure = structure
    );
    let structure_summary = ai_service.analyze_text(&structure_prompt).await.unwrap_or_else(|_| "No structure summary available.".to_string());

    // Step 2: Dynamically select up to 8 key files for detailed summary
    fn score_file(path: &str) -> i32 {
        let lower = path.to_lowercase();
        let mut score = 0;
        // Shallowest path (fewer slashes = higher score)
        score += 10 - lower.matches('/').count() as i32;
        // Name contains key words
        for kw in ["main", "index", "config", "readme", "app", "setup", "env"] {
            if lower.contains(kw) { score += 5; }
        }
        // Code/config extension
        for ext in [".rs", ".ts", ".tsx", ".js", ".jsx", ".json", ".toml", ".md", ".txt", ".yaml", ".yml", ".html", ".css", ".scss", ".mjs", ".cjs", ".env", ".lock", ".sql"] {
            if lower.ends_with(ext) { score += 2; }
        }
        score
    }
    let mut scored_files: Vec<_> = file_list.iter()
        .filter(|(_, is_dir)| !*is_dir)
        .map(|(path, _)| (path.clone(), score_file(path)))
        .collect();
    scored_files.sort_by(|a, b| b.1.cmp(&a.1));
    let key_files: Vec<_> = scored_files.into_iter().take(8).map(|(p, _)| p).collect();

    // Step 2: For each key file, get a summary from the AI
    let mut file_summaries = Vec::new();
    for path in &key_files {
        let full_path = format!("{}/{}", extracted_dir, path);
        if let Ok(content_bytes) = storage_service.read_file(&full_path).await {
            let content_bytes = if content_bytes.len() > 1000 {
                &content_bytes[..1000]
            } else {
                &content_bytes
            };
            if let Ok(content) = String::from_utf8(content_bytes.to_vec()) {
                let file_prompt = format!(
                    r#"
Here is the file `{path}` from a software project:

---
{content}
---

Please summarize what this file does, how it connects to the rest of the project, and what a junior developer should understand about it. Use Markdown formatting, clear sections, and bullet points. If the file is truncated, note that in your summary.
"#,
                    path = path,
                    content = content
                );
                let summary = ai_service.analyze_text(&file_prompt).await.unwrap_or_else(|_| format!("No summary available for {path}"));
                file_summaries.push((path.clone(), summary));
            }
        }
    }

    // Step 3: Synthesize all summaries into a final documentation prompt and get the final doc
    let mut all_summaries = String::new();
    all_summaries.push_str("# Project Structure Overview\n\n");
    all_summaries.push_str(&structure_summary);
    all_summaries.push_str("\n\n# Key File Summaries\n\n");
    let mut total_chars = all_summaries.len();
    let mut omitted_count = 0;
    for (path, summary) in &file_summaries {
        let entry = format!("## `{}`\n{}\n\n", path, summary);
        if total_chars + entry.len() < 10_000 {
            all_summaries.push_str(&entry);
            total_chars += entry.len();
        } else {
            omitted_count += 1;
        }
    }
    if omitted_count > 0 {
        all_summaries.push_str(&format!("\n--- Some file summaries omitted due to size limits ({} omitted). ---\n", omitted_count));
    }
    let final_prompt = format!(
        r#"
You are an expert technical writer, software architect, and educator. Your job is to generate the best possible documentation for this software project, specifically for junior developers and newcomers.

Below are the project structure overview and summaries of key files. Please synthesize these into a complete, beginner-friendly documentation that explains the architecture, file relationships, technology stack, developer flow, and learning tips. Use diagrams, Markdown formatting, and a welcoming, educational tone.

{all_summaries}

Now, generate the final documentation as described above.
"#,
        all_summaries = all_summaries
    );
    let final_doc = ai_service.analyze_text(&final_prompt).await.unwrap_or_else(|_| "No documentation available.".to_string());

    let doc = ProjectDocumentation {
        project_name: project_name.to_string(),
        description: final_doc,
        architecture: String::new(),
        file_analyses: Vec::new(),
        dependencies: Vec::new(),
        setup_instructions: String::new(),
    };
    Ok(HttpResponse::Ok().json(doc))
} 