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

// Helper to extract a section from markdown by heading
fn extract_markdown_section<'a>(content: &'a str, headings: &[&str]) -> Option<String> {
    let mut lines = content.lines().peekable();
    let mut in_section = false;
    let mut section = Vec::new();
    let mut current_heading_level = 0;
    while let Some(line) = lines.next() {
        let trimmed = line.trim();
        // Match heading (e.g., # Setup, ## Installation, etc.)
        if let Some((hashes, title)) = trimmed.split_once(' ') {
            if hashes.starts_with('#') {
                let level = hashes.chars().take_while(|&c| c == '#').count();
                let title_lower = title.to_lowercase();
                if headings.iter().any(|h| title_lower.contains(h)) {
                    in_section = true;
                    current_heading_level = level;
                    continue;
                }
                if in_section && level <= current_heading_level {
                    // End of section
                    break;
                }
            }
        }
        if in_section {
            section.push(line);
        }
    }
    if !section.is_empty() {
        Some(section.join("\n").trim().to_string())
    } else {
        None
    }
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

Please give a high-level architectural overview of how the folders and files relate to each other. Focus on helping a junior developer understand how this is structured and why. Include a diagram (ASCII or Mermaid if possible) that visually represents the architecture. Be concise, explicit, and do not use meta language, markdown formatting, or explanations—output only the content and diagram.
"#,
        structure = structure
    );
    let structure_summary = ai_service.analyze_text(&structure_prompt).await.unwrap_or_else(|_| "".to_string());

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
    let key_files: Vec<_> = scored_files.iter().take(8).map(|(p, _)| p.clone()).collect();

    // Step 2: For each key file, get a concise summary from the AI and build FileAnalysisDoc
    let mut file_analyses = Vec::new();
    for path in &key_files {
        let full_path = format!("{}/{}", extracted_dir, path);
        let name = path.split('/').last().unwrap_or("").to_string();
        let mut description = String::new();
        let mut dependencies = Vec::new();
        let relationships = Vec::new(); // Not implemented yet
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

Summarize in 1-2 sentences, directly and explicitly, what this file does and how it fits into the project. Do not use meta language, markdown formatting, or explanations—output only the summary.
"#,
                    path = path,
                    content = content
                );
                description = ai_service.analyze_text(&file_prompt).await.unwrap_or_else(|_| format!("No summary available for {path}"));
                // Try to parse dependencies for package.json and Cargo.toml
                if name == "package.json" {
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                        if let Some(deps) = json.get("dependencies") {
                            if let Some(obj) = deps.as_object() {
                                dependencies = obj.keys().cloned().collect();
                            }
                        }
                    }
                } else if name == "Cargo.toml" {
                    let mut deps = Vec::new();
                    for line in content.lines() {
                        if line.trim_start().starts_with("[dependencies]") {
                            for dep_line in content.lines().skip_while(|l| !l.trim_start().starts_with("[dependencies]")).skip(1) {
                                let dep_line = dep_line.trim();
                                if dep_line.starts_with('[') { break; }
                                if let Some((dep, _)) = dep_line.split_once('=') {
                                    deps.push(dep.trim().to_string());
                                }
                            }
                            break;
                        }
                    }
                    dependencies = deps;
                }
            }
        }
        file_analyses.push(FileAnalysisDoc {
            path: path.clone(),
            name,
            description,
            dependencies,
            relationships,
        });
    }

    // Step 3: Collect all dependencies from file_analyses
    let mut dependencies = Vec::new();
    for file in &file_analyses {
        for dep in &file.dependencies {
            if !dependencies.contains(dep) {
                dependencies.push(dep.clone());
            }
        }
    }

    // Step 4: Extract setup instructions from README.md if present
    let mut setup_instructions = String::new();
    let readme_path = file_list.iter().find(|(p, _)| p.to_lowercase().ends_with("readme.md"));
    if let Some((readme_rel_path, _)) = readme_path {
        let full_path = format!("{}/{}", extracted_dir, readme_rel_path);
        if let Ok(content_bytes) = storage_service.read_file(&full_path).await {
            if let Ok(content) = String::from_utf8(content_bytes) {
                // Try to extract a Setup/Installation section using markdown heading parsing
                let headings = ["setup", "installation", "getting started"];
                if let Some(section) = extract_markdown_section(&content, &headings) {
                    setup_instructions = section;
                } else {
                    setup_instructions = content;
                }
            }
        }
    }
    // If still empty, leave as empty string (frontend will handle)

    // Step 5: Synthesize all summaries into a final documentation prompt and get the final doc
    let mut all_summaries = String::new();
    all_summaries.push_str("# Project Structure Overview\n\n");
    all_summaries.push_str(&structure_summary);
    all_summaries.push_str("\n\n# Key File Summaries\n\n");
    let mut total_chars = all_summaries.len();
    let mut omitted_count = 0;
    for file in &file_analyses {
        let entry = format!("## `{}`\n{}\n\n", file.path, file.description);
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
        architecture: structure_summary,
        file_analyses,
        dependencies,
        setup_instructions,
    };
    Ok(HttpResponse::Ok().json(doc))
} 