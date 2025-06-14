import { useState, useEffect } from "react";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { Badge } from "@/components/ui/badge";
import { Skeleton } from "@/components/ui/skeleton";
import { BrainCogIcon, CodeIcon, FileTextIcon, BoxesIcon, AlertCircleIcon } from "lucide-react";
import { FileContent } from "@/components/explorer/FileContent";
import { Prism as SyntaxHighlighter } from 'react-syntax-highlighter';
import { vscDarkPlus } from 'react-syntax-highlighter/dist/esm/styles/prism';

interface AIAnalysisProps {
  filePath?: string;
  fileName?: string;
}

interface AnalysisComponent {
  name: string;
  signature?: string;
  description: string;
}

interface AnalysisDependency {
  type: string;
  name: string;
  description: string;
}

interface AnalysisRecommendation {
  title: string;
  description: string;
  priority: 'high' | 'medium' | 'low';
}

interface AnalysisData {
  summary: string;
  tags: string[];
  components: AnalysisComponent[];
  dependencies: AnalysisDependency[];
  recommendations: AnalysisRecommendation[];
}

export function AIAnalysis({ filePath, fileName }: AIAnalysisProps) {
  const [loading, setLoading] = useState(false);
  const [analysisData, setAnalysisData] = useState<AnalysisData | null>(null);
  const [fileContent, setFileContent] = useState<string | null>(null);
  const [contentLoading, setContentLoading] = useState(false);

  useEffect(() => {
    if (filePath && fileName) {
      setLoading(true);
      // Fetch file content when a file is selected
      fetchFileContent();
      // Simulate API call to analyze the file
      setTimeout(() => {
        // Mock analysis data based on file extension
        if (fileName.endsWith(".rs")) {
          setAnalysisData(rustFileAnalysis);
        } else if (fileName.endsWith(".md")) {
          setAnalysisData(markdownFileAnalysis);
        } else if (fileName.endsWith(".toml")) {
          setAnalysisData(tomlFileAnalysis);
        } else {
          setAnalysisData(defaultFileAnalysis);
        }
        setLoading(false);
      }, 1500);
    } else {
      setAnalysisData(null);
      setFileContent(null);
    }
  }, [filePath, fileName]);

  const fetchFileContent = async () => {
    if (!filePath) return;
    
    setContentLoading(true);
    try {
      // Get the project name and UUID from the path
      const pathParts = filePath.split('/').filter(Boolean); // Remove empty strings
      if (pathParts.length < 2) {
        throw new Error('Invalid file path');
      }
      
      // Get the project name (first part) and the rest of the path
      const projectName = pathParts[0];
      const relativePath = pathParts.slice(1).join('/');
      
      // Get the UUID from localStorage
      const uploadData = localStorage.getItem('uploadData');
      if (!uploadData) {
        throw new Error('No upload data found');
      }
      
      const { file_id } = JSON.parse(uploadData);
      if (!file_id) {
        throw new Error('No file ID found in upload data');
      }
      
      // Construct the full path with the UUID
      const fullPath = `extracted_${file_id}/${relativePath}`;
      
      console.log('Debug info:');
      console.log('- Original filePath:', filePath);
      console.log('- Path parts:', pathParts);
      console.log('- Project name:', projectName);
      console.log('- Relative path:', relativePath);
      console.log('- File ID:', file_id);
      console.log('- Full path:', fullPath);
      
      const url = `http://127.0.0.1:8080/api/upload/content/${encodeURIComponent(fullPath)}`;
      console.log('- Request URL:', url);
      
      const response = await fetch(url, {
        headers: {
          'Accept': 'text/plain',
        },
      });
      
      if (!response.ok) {
        const errorText = await response.text();
        console.error('Server response:', errorText);
        console.error('Response status:', response.status);
        console.error('Response headers:', Object.fromEntries(response.headers.entries()));
        throw new Error(`Failed to fetch file content: ${response.status} ${response.statusText}`);
      }
      
      const contentType = response.headers.get('content-type');
      console.log('Response content type:', contentType);
      
      const content = await response.text();
      console.log('Received content length:', content.length);
      console.log('Content preview:', content.substring(0, 100));
      
      setFileContent(content);
    } catch (error) {
      console.error('Error fetching file content:', error);
      setFileContent(null);
    } finally {
      setContentLoading(false);
    }
  };

  if (!filePath || !fileName) {
    return (
      <div className="flex flex-col items-center justify-center h-full p-6 text-center">
        <BrainCogIcon className="h-16 w-16 text-muted-foreground mb-4 opacity-50" />
        <h3 className="text-xl font-medium mb-2">Select a File to Analyze</h3>
        <p className="text-muted-foreground">
          Choose a file from the explorer to see AI-powered insights.
        </p>
      </div>
    );
  }

  if (loading) {
    return (
      <div className="p-4 space-y-4">
        <div className="flex items-center space-x-4 mb-6">
          <Skeleton className="h-10 w-10 rounded" />
          <div className="space-y-2">
            <Skeleton className="h-5 w-40" />
            <Skeleton className="h-4 w-20" />
          </div>
        </div>
        <Skeleton className="h-4 w-full" />
        <Skeleton className="h-4 w-[90%]" />
        <Skeleton className="h-4 w-[85%]" />
        <div className="pt-4">
          <Skeleton className="h-8 w-40 mb-4" />
          <Skeleton className="h-20 w-full rounded" />
        </div>
      </div>
    );
  }

  if (!analysisData) {
    return (
      <div className="flex flex-col items-center justify-center h-full p-6 text-center">
        <AlertCircleIcon className="h-16 w-16 text-muted-foreground mb-4 opacity-50" />
        <h3 className="text-xl font-medium mb-2">Analysis Not Available</h3>
        <p className="text-muted-foreground">
          We couldn't generate analysis for this file type.
        </p>
      </div>
    );
  }

  return (
    <div className="p-4">
      <div className="flex items-start mb-6">
        <div className="bg-zipmind-100/10 p-2 rounded mr-4">
          {fileName.endsWith(".rs") && <CodeIcon className="h-8 w-8 text-zipmind-400" />}
          {fileName.endsWith(".md") && <FileTextIcon className="h-8 w-8 text-zipmind-400" />}
          {fileName.endsWith(".toml") && <BoxesIcon className="h-8 w-8 text-zipmind-400" />}
          {!fileName.endsWith(".rs") && !fileName.endsWith(".md") && !fileName.endsWith(".toml") && <FileTextIcon className="h-8 w-8 text-zipmind-400" />}
        </div>
        <div>
          <h2 className="text-xl font-semibold">{fileName}</h2>
          <p className="text-sm text-muted-foreground">{filePath}</p>
        </div>
      </div>

      <Tabs defaultValue="summary" className="w-full">
        <TabsList className="mb-4">
          <TabsTrigger value="summary">Summary</TabsTrigger>
          <TabsTrigger value="contents" onClick={fetchFileContent}>File Contents</TabsTrigger>
          <TabsTrigger value="components">Components</TabsTrigger>
          <TabsTrigger value="dependencies">Dependencies</TabsTrigger>
          <TabsTrigger value="recommendations">Recommendations</TabsTrigger>
        </TabsList>
        
        <TabsContent value="summary" className="space-y-4">
          <p className="text-sm leading-relaxed">{analysisData.summary}</p>
          <div className="flex flex-wrap gap-2 mt-2">
            {analysisData.tags.map((tag: string) => (
              <Badge key={tag} variant="outline">{tag}</Badge>
            ))}
          </div>
        </TabsContent>
        
        <TabsContent value="contents" className="space-y-4">
          {contentLoading ? (
            <div className="space-y-2">
              <Skeleton className="h-4 w-full" />
              <Skeleton className="h-4 w-[90%]" />
              <Skeleton className="h-4 w-[85%]" />
            </div>
          ) : fileContent ? (
            <div className="relative">
              <SyntaxHighlighter
                language={getLanguageFromFileName(fileName)}
                style={vscDarkPlus}
                customStyle={{
                  margin: 0,
                  borderRadius: '0.5rem',
                  maxHeight: '600px',
                }}
                showLineNumbers
                wrapLines
              >
                {fileContent}
              </SyntaxHighlighter>
              <div className="absolute top-2 right-2">
                <Badge variant="secondary">Read Only</Badge>
              </div>
            </div>
          ) : (
            <div className="text-center py-8 text-muted-foreground">
              <FileTextIcon className="h-8 w-8 mx-auto mb-2 opacity-50" />
              <p>No content available</p>
            </div>
          )}
        </TabsContent>
        
        <TabsContent value="components">
          <div className="space-y-4">
            {analysisData.components.map((component: AnalysisComponent, index: number) => (
              <Card key={index}>
                <CardHeader className="py-3">
                  <CardTitle className="text-base">{component.name}</CardTitle>
                  {component.signature && (
                    <CardDescription className="font-mono text-xs bg-code p-2 rounded overflow-x-auto">
                      {component.signature}
                    </CardDescription>
                  )}
                </CardHeader>
                <CardContent>
                  <p className="text-sm">{component.description}</p>
                </CardContent>
              </Card>
            ))}
          </div>
        </TabsContent>
        
        <TabsContent value="dependencies">
          <div className="space-y-4">
            {analysisData.dependencies.map((dep: AnalysisDependency, index: number) => (
              <div key={index} className="flex items-start">
                <Badge className="mt-0.5 mr-2">{dep.type}</Badge>
                <div>
                  <h4 className="font-medium">{dep.name}</h4>
                  <p className="text-sm text-muted-foreground">{dep.description}</p>
                </div>
              </div>
            ))}
          </div>
        </TabsContent>
        
        <TabsContent value="recommendations">
          <div className="space-y-4">
            {analysisData.recommendations.map((rec: AnalysisRecommendation, index: number) => (
              <Card key={index}>
                <CardHeader className="py-3">
                  <CardTitle className="text-base flex items-center gap-2">
                    {rec.priority === "high" && <Badge className="bg-destructive">High Priority</Badge>}
                    {rec.priority === "medium" && <Badge className="bg-amber-500">Medium Priority</Badge>}
                    {rec.priority === "low" && <Badge className="bg-green-500">Low Priority</Badge>}
                    {rec.title}
                  </CardTitle>
                </CardHeader>
                <CardContent>
                  <p className="text-sm">{rec.description}</p>
                </CardContent>
              </Card>
            ))}
          </div>
        </TabsContent>
      </Tabs>
    </div>
  );
}

// Mock Analysis Data
const rustFileAnalysis: AnalysisData = {
  summary: "This file defines the authentication routes and handlers for the application. It implements user registration, login, and session management using JWT tokens. Password hashing is performed with bcrypt, and input validation uses the validator crate.",
  tags: ["Authentication", "JWT", "Rust", "API Routes", "Security"],
  components: [
    {
      name: "register_user",
      signature: "async fn register_user(Json(payload): Json<RegisterRequest>) -> Result<impl Reply, Rejection>",
      description: "Creates a new user account, validates the input, hashes the password, and stores the user in the database."
    },
    {
      name: "login_user",
      signature: "async fn login_user(Json(payload): Json<LoginRequest>) -> Result<impl Reply, Rejection>",
      description: "Authenticates a user by email and password, generates a JWT token, and returns it to the client."
    },
    {
      name: "verify_token",
      signature: "async fn verify_token(token: &str) -> Result<Claims, AuthError>",
      description: "Validates a JWT token and extracts the user claims from it."
    }
  ],
  dependencies: [
    {
      type: "crate",
      name: "jsonwebtoken",
      description: "Used for JWT token generation and verification"
    },
    {
      type: "crate",
      name: "bcrypt",
      description: "Used for password hashing and verification"
    },
    {
      type: "internal",
      name: "models::user",
      description: "User data model and database operations"
    }
  ],
  recommendations: [
    {
      title: "Add Rate Limiting",
      description: "Implement rate limiting on login endpoints to prevent brute force attacks.",
      priority: "high"
    },
    {
      title: "Token Refresh Mechanism",
      description: "Add a token refresh mechanism to improve user experience without compromising security.",
      priority: "medium"
    }
  ]
};

const markdownFileAnalysis: AnalysisData = {
  summary: "This markdown file serves as the project's README documentation. It provides an overview of the project, installation instructions, usage examples, and contribution guidelines. The document is well-structured with clear sections.",
  tags: ["Documentation", "Markdown", "README", "Project Info"],
  components: [
    {
      name: "Project Overview",
      description: "Explains the purpose and core features of the project."
    },
    {
      name: "Installation Guide",
      description: "Step-by-step instructions for installing the software and its dependencies."
    },
    {
      name: "API Documentation",
      description: "Details about the REST API endpoints, parameters, and response formats."
    }
  ],
  dependencies: [
    {
      type: "reference",
      name: "External API",
      description: "References to third-party services or APIs used in the project"
    },
    {
      type: "internal",
      name: "Configuration Files",
      description: "Mentions of configuration files in the project"
    }
  ],
  recommendations: [
    {
      title: "Add Code Examples",
      description: "Include more code examples showing how to use the library in different scenarios.",
      priority: "medium"
    },
    {
      title: "Update API Documentation",
      description: "The API endpoints documentation seems to be outdated compared to the current implementation.",
      priority: "high"
    }
  ]
};

const tomlFileAnalysis: AnalysisData = {
  summary: "This is the Cargo.toml configuration file for a Rust project. It defines the package metadata, dependencies, build settings, and other configuration options for the project. The project appears to be a web service using the axum framework with database connectivity.",
  tags: ["Configuration", "Cargo", "Dependencies", "Rust", "TOML"],
  components: [
    {
      name: "Package Information",
      description: "Defines the package name, version, authors, and other metadata."
    },
    {
      name: "Dependencies Section",
      description: "Lists all the direct dependencies required by the project."
    },
    {
      name: "Build Configuration",
      description: "Specifies build settings like optimization levels and target features."
    }
  ],
  dependencies: [
    {
      type: "dependency",
      name: "axum",
      description: "Web framework used for handling HTTP requests and routing"
    },
    {
      type: "dependency",
      name: "tokio",
      description: "Asynchronous runtime for the application"
    },
    {
      type: "dependency",
      name: "sqlx",
      description: "Database access library for PostgreSQL"
    }
  ],
  recommendations: [
    {
      title: "Version Constraints",
      description: "Some dependencies don't have version constraints, which might lead to compatibility issues in the future.",
      priority: "medium"
    },
    {
      title: "Consider Feature Flags",
      description: "Use feature flags to make optional components configurable at compile time.",
      priority: "low"
    }
  ]
};

const defaultFileAnalysis: AnalysisData = {
  summary: "This file contains generic content that couldn't be specifically analyzed. It appears to be a supporting file in the project structure.",
  tags: ["Unknown", "Supporting File"],
  components: [
    {
      name: "Unknown Components",
      description: "The file structure couldn't be parsed into specific components."
    }
  ],
  dependencies: [
    {
      type: "unknown",
      name: "Indeterminate Dependencies",
      description: "Dependencies couldn't be accurately determined from this file type."
    }
  ],
  recommendations: [
    {
      title: "Add File Documentation",
      description: "Consider adding documentation comments to clarify the purpose of this file.",
      priority: "low"
    }
  ]
};

function getLanguageFromFileName(fileName: string): string {
  const extension = fileName.split('.').pop()?.toLowerCase();
  switch (extension) {
    case 'rs':
      return 'rust';
    case 'py':
      return 'python';
    case 'js':
    case 'jsx':
      return 'javascript';
    case 'ts':
    case 'tsx':
      return 'typescript';
    case 'html':
      return 'html';
    case 'css':
      return 'css';
    case 'json':
      return 'json';
    case 'md':
      return 'markdown';
    case 'toml':
      return 'toml';
    case 'yaml':
    case 'yml':
      return 'yaml';
    case 'sh':
      return 'bash';
    case 'sql':
      return 'sql';
    default:
      return 'text';
  }
}
