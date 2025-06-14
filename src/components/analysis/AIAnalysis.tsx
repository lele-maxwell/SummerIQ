import { useState, useEffect } from "react";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { Badge } from "@/components/ui/badge";
import { Skeleton } from "@/components/ui/skeleton";
import { BrainCogIcon, CodeIcon, FileTextIcon, BoxesIcon, AlertCircleIcon, RefreshCwIcon } from "lucide-react";
import { FileContent } from "@/components/explorer/FileContent";
import { Prism as SyntaxHighlighter } from 'react-syntax-highlighter';
import { vscDarkPlus } from 'react-syntax-highlighter/dist/esm/styles/prism';
import { Button } from "@/components/ui/button";
import { ScrollArea } from "@/components/ui/scroll-area";

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

interface FileAnalysis {
  purpose: string;
  keyComponents: string[];
  dependencies: string[];
  qualityInsights: string[];
  lastUpdated: string;
}

const rustFileAnalysis: AnalysisData = {
  summary: "Rust source file implementing core functionality",
  tags: ["rust", "backend", "api"],
  components: [
    {
      name: "main",
      signature: "fn main() -> Result<()>",
      description: "Entry point of the application"
    }
  ],
  dependencies: [
    {
      type: "crate",
      name: "actix-web",
      description: "Web framework for Rust"
    }
  ],
  recommendations: [
    {
      title: "Add Error Handling",
      description: "Implement proper error handling for all operations",
      priority: "high"
    }
  ]
};

const markdownFileAnalysis: AnalysisData = {
  summary: "Documentation file with project information",
  tags: ["documentation", "markdown"],
  components: [],
  dependencies: [],
  recommendations: [
    {
      title: "Add Code Examples",
      description: "Include code examples for better understanding",
      priority: "medium"
    }
  ]
};

const tomlFileAnalysis: AnalysisData = {
  summary: "Project configuration file",
  tags: ["config", "toml"],
  components: [],
  dependencies: [],
  recommendations: [
    {
      title: "Add Comments",
      description: "Add comments to explain configuration options",
      priority: "low"
    }
  ]
};

const defaultFileAnalysis: AnalysisData = {
  summary: "General file analysis",
  tags: ["file"],
  components: [],
  dependencies: [],
  recommendations: [
    {
      title: "Review File",
      description: "Review file contents for potential improvements",
      priority: "medium"
    }
  ]
};

export function AIAnalysis({ filePath, fileName }: AIAnalysisProps) {
  const [activeTab, setActiveTab] = useState("contents");
  const [fileContent, setFileContent] = useState<string | null>(null);
  const [contentLoading, setContentLoading] = useState(false);
  const [analysisLoading, setAnalysisLoading] = useState(false);
  const [fileAnalysis, setFileAnalysis] = useState<FileAnalysis | null>(null);
  const [analysisError, setAnalysisError] = useState<string | null>(null);
  const [analysisData, setAnalysisData] = useState<AnalysisData | null>(null);
  const [loading, setLoading] = useState(false);

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

  const fetchFileAnalysis = async () => {
    if (!filePath) return;
    
    setAnalysisLoading(true);
    setAnalysisError(null);
    try {
      const response = await fetch(`http://127.0.0.1:8080/api/analysis/file/${encodeURIComponent(filePath)}`, {
        headers: {
          'Accept': 'application/json',
        },
      });
      
      if (!response.ok) {
        throw new Error(`Failed to fetch analysis: ${response.status} ${response.statusText}`);
      }
      
      const analysis = await response.json();
      setFileAnalysis({
        purpose: analysis.language || "No purpose provided",
        keyComponents: Array.isArray(analysis.components) ? analysis.components : [],
        dependencies: Array.isArray(analysis.dependencies) ? analysis.dependencies : [],
        qualityInsights: Array.isArray(analysis.recommendations) ? analysis.recommendations : [],
        lastUpdated: analysis.analysis_time || new Date().toISOString(),
      });
    } catch (error) {
      console.error('Error fetching file analysis:', error);
      setAnalysisError(error instanceof Error ? error.message : 'Failed to fetch analysis');
    } finally {
      setAnalysisLoading(false);
    }
  };

  useEffect(() => {
    if (filePath) {
      fetchFileContent();
      fetchFileAnalysis();
    }
  }, [filePath]);

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
    <Card className="w-full">
      <CardHeader>
        <CardTitle className="flex items-center gap-2">
          <FileTextIcon className="h-5 w-5" />
          {fileName}
        </CardTitle>
        <CardDescription>
          {filePath}
        </CardDescription>
      </CardHeader>
      <CardContent>
        <Tabs value={activeTab} onValueChange={setActiveTab} className="w-full">
          <TabsList className="grid w-full grid-cols-3">
            <TabsTrigger value="contents" className="flex items-center gap-2">
              <CodeIcon className="h-4 w-4" />
              Contents
            </TabsTrigger>
            <TabsTrigger value="analysis" className="flex items-center gap-2">
              <BrainCogIcon className="h-4 w-4" />
              AI Analysis
            </TabsTrigger>
            <TabsTrigger value="dependencies" className="flex items-center gap-2">
              <BoxesIcon className="h-4 w-4" />
              Dependencies
            </TabsTrigger>
          </TabsList>

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

          <TabsContent value="analysis" className="space-y-4">
            {analysisLoading ? (
              <div className="space-y-2">
                <Skeleton className="h-4 w-full" />
                <Skeleton className="h-4 w-[90%]" />
                <Skeleton className="h-4 w-[85%]" />
              </div>
            ) : analysisError ? (
              <div className="text-center py-8 text-destructive">
                <AlertCircleIcon className="h-8 w-8 mx-auto mb-2" />
                <p>{analysisError}</p>
                <Button 
                  variant="outline" 
                  className="mt-4"
                  onClick={fetchFileAnalysis}
                >
                  <RefreshCwIcon className="h-4 w-4 mr-2" />
                  Retry Analysis
                </Button>
              </div>
            ) : fileAnalysis ? (
              <ScrollArea className="h-[600px] pr-4">
                <div className="space-y-6">
                  <div>
                    <h3 className="text-lg font-semibold mb-2">File Purpose</h3>
                    <p className="text-muted-foreground">{fileAnalysis.purpose}</p>
                  </div>
                  
                  <div>
                    <h3 className="text-lg font-semibold mb-2">Key Components</h3>
                    <ul className="list-disc list-inside space-y-1 text-muted-foreground">
                      {Array.isArray(fileAnalysis.keyComponents) && fileAnalysis.keyComponents.map((component, index) => (
                        <li key={index}>{component}</li>
                      ))}
                    </ul>
                  </div>
                  
                  <div>
                    <h3 className="text-lg font-semibold mb-2">Dependencies</h3>
                    <div className="flex flex-wrap gap-2">
                      {Array.isArray(fileAnalysis.dependencies) && fileAnalysis.dependencies.map((dep, index) => (
                        <Badge key={index} variant="secondary">{dep}</Badge>
                      ))}
                    </div>
                  </div>

                  <div>
                    <h3 className="text-lg font-semibold mb-2">Code Quality Insights</h3>
                    <ul className="list-disc list-inside space-y-1 text-muted-foreground">
                      {Array.isArray(fileAnalysis.qualityInsights) && fileAnalysis.qualityInsights.map((insight, index) => (
                        <li key={index}>{insight}</li>
                      ))}
                    </ul>
                  </div>
                  
                  <div className="text-sm text-muted-foreground">
                    Last updated: {new Date(fileAnalysis.lastUpdated).toLocaleString()}
                  </div>
                  
                  <Button 
                    variant="outline" 
                    className="w-full"
                    onClick={fetchFileAnalysis}
                  >
                    <RefreshCwIcon className="h-4 w-4 mr-2" />
                    Refresh Analysis
                  </Button>
                </div>
              </ScrollArea>
            ) : (
              <div className="text-center py-8 text-muted-foreground">
                <BrainCogIcon className="h-8 w-8 mx-auto mb-2 opacity-50" />
                <p>No analysis available</p>
              </div>
            )}
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
        </Tabs>
      </CardContent>
    </Card>
  );
}

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
