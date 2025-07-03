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

    // Build a file/folder structure string for the prompt
    let mut structure = String::new();
    for (path, is_dir) in &file_list {
        if *is_dir {
            structure.push_str(&format!("[DIR] {}\n", path));
        } else {
            structure.push_str(&format!("      {}\n", path));
        }
    }

    // Helper to check if a file is likely text/code (not binary)
    fn is_text_file(path: &str) -> bool {
        let text_exts = [
            ".rs", ".ts", ".tsx", ".js", ".jsx", ".json", ".toml", ".md", ".txt", ".yaml", ".yml", ".html", ".css", ".scss", ".mjs", ".cjs", ".env", ".lock", ".sql"
        ];
        text_exts.iter().any(|ext| path.ends_with(ext))
    }

    // Gather file contents for up to 10 key files, truncate each to 1000 bytes, and limit total prompt size
    let mut file_contents = String::new();
    let mut included_files = 0;
    let mut omitted_files = Vec::new();
    let mut total_chars = structure.len();
    for (path, is_dir) in &file_list {
        if *is_dir { continue; }
        if !is_text_file(path) { continue; }
        let is_key = key_file_names.iter().any(|k| path.ends_with(k));
        if is_key && included_files < 10 && total_chars < 15000 {
            let full_path = format!("{}/{}", extracted_dir, path);
            if let Ok(content_bytes) = storage_service.read_file(&full_path).await {
                // Truncate to 1000 bytes
                let content_bytes = if content_bytes.len() > 1000 {
                    &content_bytes[..1000]
                } else {
                    &content_bytes
                };
                if let Ok(content) = String::from_utf8(content_bytes.to_vec()) {
                    let entry = format!("--- {} ---\n{}\n\n", path, content);
                    if total_chars + entry.len() < 15000 {
                        file_contents.push_str(&entry);
                        included_files += 1;
                        total_chars += entry.len();
                    } else {
                        omitted_files.push(path.clone());
                    }
                }
            }
        } else {
            omitted_files.push(path.clone());
        }
    }
    if !omitted_files.is_empty() {
        file_contents.push_str("\n--- Some files omitted or truncated due to size limits. ---\n");
        for path in omitted_files.iter().take(10) {
            file_contents.push_str(&format!("[omitted] {}\n", path));
        }
        if omitted_files.len() > 10 {
            file_contents.push_str(&format!("...and {} more omitted files.\n", omitted_files.len() - 10));
        }
    }

    // Build the AI prompt using a highly directive, educational, and thorough instruction
    let prompt = format!(
        r#"
You are an expert technical writer, software architect, and educator. Your job is to generate the best possible documentation for this software project, specifically for junior developers and newcomers.

## Instructions:
- Carefully analyze the full file/folder structure and the content of each file below.
- Identify how files and modules relate to each other, and how data flows through the system.
- Explain the overall architecture, the purpose of each major component, and how they interact.
- Use diagrams (Mermaid, ASCII, or Markdown tables) to illustrate architecture and relationships.
- For each major file/folder, explain:
    - Its purpose and role in the project.
    - How it connects to other files/folders.
    - Whether it is an entry point, config, model, route, or utility.
- List and explain all technologies and frameworks used, with beginner-friendly resources.
- Walk through a typical developer flow (e.g., sign up, upload, analyze, view results).
- Use clear, beginner-friendly language. Define technical terms and break down complex concepts.
- Use analogies, examples, and "What to Learn Next" tips.
- Write in an encouraging, welcoming tone that empowers juniors to contribute.
- Format everything in Markdown with clear sections, headings, bullet points, and code blocks.
- **Do not start writing until you have thoroughly analyzed the entire project.**

## Project Name:
{project_name}

## File/Folder Structure:
{structure}

## File Contents:
{file_contents}

## Now, generate the documentation as described above.
"#,
        project_name = project_name,
        structure = structure,
        file_contents = file_contents
    );

    // Call the AI once with the full prompt
    let ai_doc = ai_service.analyze_text(&prompt).await.unwrap_or_else(|_| "No documentation available.".to_string());

    let doc = ProjectDocumentation {
        project_name: project_name.to_string(),
        description: ai_doc,
        architecture: String::new(),
        file_analyses: Vec::new(),
        dependencies: Vec::new(),
        setup_instructions: String::new(),
    };
    Ok(HttpResponse::Ok().json(doc))
} 