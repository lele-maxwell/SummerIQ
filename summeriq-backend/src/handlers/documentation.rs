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

    // Gather file contents (up to 10KB per file, skip binaries)
    let mut file_contents = String::new();
    for (path, is_dir) in &file_list {
        if *is_dir { continue; }
        if !is_text_file(path) { continue; }
        let full_path = format!("{}/{}", extracted_dir, path);
        if let Ok(content_bytes) = storage_service.read_file(&full_path).await {
            // Limit to 10KB per file
            let content_bytes = if content_bytes.len() > 10_240 {
                &content_bytes[..10_240]
            } else {
                &content_bytes
            };
            if let Ok(content) = String::from_utf8(content_bytes.to_vec()) {
                file_contents.push_str(&format!("--- {} ---\n{}\n\n", path, content));
            }
        }
    }

    // Build the AI prompt using the user's requirements
    let prompt = format!(
        r#"
You are an expert technical writer and software architect. Your task is to generate well-structured, clean, and educational documentation for this software project, specifically tailored for junior developers and newcomers to the tech stack.

Project Name: {project_name}

Here is the file and folder structure of the project:
{structure}

Here are the contents of the project files:
{file_contents}

✅ Documentation Goals:

🎯 Target Audience:
Junior developers who are eager to learn, understand, and confidently contribute to the project.
📚 Documentation Requirements:
1. Project Architecture Overview
    Provide a clear, high-level explanation of the system.
    Include diagrammatic representations (ASCII, Mermaid, or visual if supported).
    Show how each major component (frontend, backend, database, storage, etc.) interacts.
    Emphasize flow of data and responsibilities of each layer (e.g., API calls, auth, storage).
2. File and Folder Structure Walkthrough
    List and explain all major files and folders (e.g., main.rs, routes/, models/, App.tsx).
    For each file:
        Explain its purpose.
        How it connects to other parts of the system.
        Whether it's an entry point, config, model, route, or utility.
    Use clear, beginner-friendly language and define technical terms.
3. Technology Stack Summary
    List all the technologies and frameworks used in the project (e.g., Rust, Axum, SQLx, JWT, MinIO, PostgreSQL, React, TailwindCSS).
    For each:
        Explain what it's used for in the project.
        Include 1–2 beginner-friendly learning resources (YouTube links, blog posts, official docs).
4. Developer Flow & Use Case Walkthrough
    Describe the user journey through the app:
        Example: User signs up → logs in → uploads a file → sees it in dashboard → logs out.
    Explain:
        Authentication flow (e.g., JWT, session tokens).
        Routing and navigation logic (e.g., protected routes).
        API interactions, form handling, file uploads, and error responses.
    Show how the frontend and backend communicate.
5. Educational and Encouraging Tone
    Write as if teaching a junior developer.
    Break down complex concepts with analogies, examples, and clear definitions.
    Use a friendly, welcoming tone that encourages exploration and learning.
    Include 'What to Learn Next' tips for further growth.
🌱 Bonus Outcome:
    This documentation should give any junior developer enough confidence to understand and meaningfully contribute to the codebase, architecture, and project decisions.

**Format your entire output using Markdown. Use headings, subheadings, bullet points, code blocks, and diagrams where appropriate.**
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