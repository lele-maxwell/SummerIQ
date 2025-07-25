import React, { useEffect, useState } from "react";
import { DownloadIcon, Sun, Moon } from "lucide-react";
import { FileNode } from "@/components/explorer/types";
import { documentationApi, ProjectDocumentation } from "@/api/documentation";
import ReactMarkdown from 'react-markdown';
import remarkGfm from 'remark-gfm';
import rehypeHighlight from 'rehype-highlight';
import 'highlight.js/styles/github.css';
import MermaidRenderer from '@/components/MermaidRenderer';

// Custom renderer for code blocks to support Mermaid diagrams
const renderers = {
  code({inline, className, children, ...props}: any) {
    const code = String(children).trim();
    if (className === 'language-mermaid' || code.startsWith('graph ')) {
      return <MermaidRenderer chart={code} />;
    }
    return <code className={className} {...props}>{children}</code>;
  }
};

// Extract the first mermaid code block or graph definition from text
function extractMermaidDiagram(text: string): string | null {
  // Try to find a ```mermaid code block
  const codeBlockMatch = text.match(/```mermaid\s*([\s\S]*?)```/);
  if (codeBlockMatch) {
    return codeBlockMatch[1].trim();
  }
  // Fallback: find the first occurrence of 'graph' or 'flowchart' and collect until a blank line or a line starting with a non-indented character
  const lines = text.split('\n');
  let startIdx = lines.findIndex(line => line.trim().startsWith('graph ') || line.trim().startsWith('flowchart '));
  if (startIdx !== -1) {
    let diagram = lines[startIdx];
    for (let i = startIdx + 1; i < lines.length; i++) {
      // Stop at blank line or a line that looks like a new section (not indented and not part of the diagram)
      if (lines[i].trim() === '' || /^[A-Za-z0-9#*_\-]/.test(lines[i])) {
        break;
      }
      diagram += '\n' + lines[i];
    }
    return diagram.trim();
  }
  return null;
}

// Helper to apply theme
function applyTheme(isDark: boolean) {
  document.documentElement.classList.remove('dark', 'light');
  document.documentElement.classList.add(isDark ? 'dark' : 'light');
}

const Architecture = () => {
  const [projectDoc, setProjectDoc] = useState<ProjectDocumentation | null>(null);
  const [uploadedFileName, setUploadedFileName] = useState<string>("");
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string>("");
  const [darkMode, setDarkMode] = useState(true);

  useEffect(() => {
    const storedFileName = localStorage.getItem('uploadedFileName');
    if (storedFileName) setUploadedFileName(storedFileName);
    setIsLoading(true);
    setError("");

    // LocalStorage cache key
    const cacheKey = storedFileName ? `projectDocCache_${storedFileName}` : null;
    if (cacheKey) {
      const cached = localStorage.getItem(cacheKey);
      if (cached) {
        try {
          setProjectDoc(JSON.parse(cached));
          setIsLoading(false);
          return;
        } catch (e) {
          // If cache is corrupted, ignore and fetch from API
        }
      }
    }

    documentationApi.getProjectDocumentation(storedFileName || "current")
      .then((doc: ProjectDocumentation) => {
        setProjectDoc(doc);
        if (cacheKey) {
          localStorage.setItem(cacheKey, JSON.stringify(doc));
        }
      })
      .catch(() => {
        setError("Failed to load project documentation. Please try again later.");
      })
      .finally(() => setIsLoading(false));
    // Set initial theme on mount
    applyTheme(darkMode);
  }, []);

  // Update theme when darkMode changes
  useEffect(() => {
    applyTheme(darkMode);
  }, [darkMode]);

  const handleDownload = () => {
    // TODO: Implement download functionality for project documentation above the alert in handleDownload
    alert("Download functionality coming soon!");
  };

  const toggleTheme = () => setDarkMode((prev) => !prev);

  if (isLoading) {
    return (
      <div className="container mx-auto py-12 text-center">
        <h1 className="text-3xl font-bold mb-4">Loading Project Architecture...</h1>
        <p className="text-muted-foreground">Please wait while we generate your project documentation.</p>
        <div className="mt-8 flex justify-center">
          <span className="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-zipmind-400"></span>
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="container mx-auto py-12 text-center">
        <h1 className="text-3xl font-bold mb-4">Error</h1>
        <p className="text-red-500">{error}</p>
      </div>
    );
  }

  if (!projectDoc || !uploadedFileName) {
    return (
      <div className="container mx-auto py-12 text-center">
        <h1 className="text-3xl font-bold mb-4">No Project Data</h1>
        <p className="text-muted-foreground">Please upload a project first to view its architecture documentation.</p>
      </div>
    );
  }

  return (
    <div className="container mx-auto py-12"> 
      {/* Back to Dashboard button */}
      <div className="mb-6 flex justify-start">
        <a
          href="/dashboard"
          className="inline-flex items-center gap-2 px-4 py-2 rounded-lg bg-gray-200 text-gray-800 font-semibold shadow hover:bg-gray-300 transition-colors duration-200"
        >
          ← Back to Dashboard
        </a>
      </div>
      <div className="flex justify-between items-center mb-8">
        <h1 className="text-4xl font-bold text-center w-full">Project Architecture: {projectDoc.project_name || uploadedFileName}</h1>
        <div className="flex gap-2 ml-4">
          <button
            className="inline-flex items-center gap-2 px-5 py-2 rounded-lg bg-gradient-to-r from-zipmind-400 to-zipmind-600 text-white font-semibold shadow-lg hover:scale-105 transition-transform duration-200"
            onClick={handleDownload}
          >
            <DownloadIcon className="w-5 h-5" />
            Download
          </button>
          <button
            className="inline-flex items-center gap-2 px-3 py-2 rounded-lg bg-gray-700 text-white font-semibold shadow hover:scale-105 transition-transform duration-200"
            onClick={toggleTheme}
            title={darkMode ? "Switch to Light Mode" : "Switch to Dark Mode"}
          >
            {darkMode ? <Sun className="w-5 h-5" /> : <Moon className="w-5 h-5" />}
          </button>
        </div>
      </div>

      {/* 1. Introduction */}
      <div className="doc-section-card mb-12">
        <h2 className="text-2xl font-semibold mb-2">Introduction</h2>
        <div className="documentation-markdown text-muted-foreground mb-4">
          <ReactMarkdown
            children={projectDoc.description || 'No project description available.'}
            remarkPlugins={[remarkGfm /*, remarkMermaid */]}
            rehypePlugins={[rehypeHighlight]}
          />
        </div>
      </div>

      {/* 2. High-Level Architecture Diagram */}
      <div className="doc-section-card mb-16">
        <h2 className="text-3xl font-bold mb-4 text-center">High-Level Architecture</h2>
        <div className="documentation-markdown text-lg text-muted-foreground text-center mb-8">
          {/* Only render the diagram if found; do not render the raw code as Markdown */}
          {(() => {
            const diagram = extractMermaidDiagram(projectDoc.architecture || '');
            if (diagram) {
              return <MermaidRenderer chart={diagram} />;
            }
            return (
              <ReactMarkdown
                children={projectDoc.architecture || 'No architecture summary available.'}
                remarkPlugins={[remarkGfm /*, remarkMermaid */]}
                rehypePlugins={[rehypeHighlight]}
                components={renderers}
              />
            );
          })()}
        </div>
      </div>

      {/* 3. Folder & File Overview */}
      <div className="doc-section-card mb-12">
        <h2 className="text-2xl font-semibold mb-2">Folder & File Overview</h2>
        <p className="text-muted-foreground mb-4">
          This section explains the purpose of each major folder and file in your uploaded project. Understanding this structure will help you navigate, learn, and contribute with confidence.
        </p>
        <div className="space-y-6">
          {projectDoc?.file_analyses?.map((file) => (
            <div key={file.path} className="bg-card rounded-lg p-4 shadow">
              <h3 className="text-lg font-bold">{file.path}</h3>
              <ul className="list-disc pl-6 text-muted-foreground mb-2">
                <li>{file.name}</li>
                <li>{file.description}</li>
                {file.dependencies?.length > 0 && (
                  <li><strong>Dependencies:</strong> {file.dependencies.join(", ")}</li>
                )}
                {file.relationships?.length > 0 && (
                  <li><strong>Relationships:</strong>
                    <ul className="list-disc pl-6">
                      {file.relationships.map((rel, idx) => (
                        <li key={idx}>{rel.target_file} ({rel.relationship_type}): {rel.description}</li>
                      ))}
                    </ul>
                  </li>
                )}
              </ul>
            </div>
          ))}
        </div>
      </div>

      {/* 4. Dependencies */}
      <div className="doc-section-card mb-12">
        <h2 className="text-2xl font-semibold mb-2">Project Dependencies</h2>
        <ul className="list-disc pl-6 text-muted-foreground mb-4">
          {projectDoc?.dependencies?.length > 0 ? (
            projectDoc.dependencies.map((dep, idx) => (
              <li key={idx}>{dep}</li>
            ))
          ) : (
            <li>No dependencies found.</li>
          )}
        </ul>
      </div>

      {/* 5. Setup Instructions */}
      <div className="doc-section-card mb-12">
        <h2 className="text-2xl font-semibold mb-2">Setup Instructions</h2>
        <div className="documentation-markdown text-muted-foreground mb-4">
          <ReactMarkdown
            children={projectDoc?.setup_instructions || 'No setup instructions available.'}
            remarkPlugins={[remarkGfm /*, remarkMermaid */]}
            rehypePlugins={[rehypeHighlight]}
          />
        </div>
      </div>
    </div>
  );
};

export default Architecture; 