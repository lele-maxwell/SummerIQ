import React, { useState, useEffect } from 'react';
import { Card, CardHeader, CardTitle, CardDescription, CardContent, Tabs, TabsList, TabsTrigger, TabsContent, ScrollArea, Skeleton, Badge, Button, SyntaxHighlighter } from '@/components/ui';
import { FileTextIcon, CodeIcon, BrainCogIcon, BoxesIcon, AlertCircleIcon, RefreshCwIcon } from 'lucide-react';
import { vscDarkPlus } from '@/lib/prism-themes';

interface FileAnalysis {
  language: string;
  file_purpose: string;
  dependencies: string[];
  analysis_time: string;
  contents: string;
}

interface AIAnalysisProps {
  filePath?: string;
  fileName?: string;
}

export function AIAnalysis({ filePath, fileName }: AIAnalysisProps) {
  const [activeTab, setActiveTab] = useState("contents");
  const [fileContent, setFileContent] = useState<string | null>(null);
  const [contentLoading, setContentLoading] = useState(false);
  const [analysisLoading, setAnalysisLoading] = useState(false);
  const [fileAnalysis, setFileAnalysis] = useState<FileAnalysis | null>(null);
  const [analysisError, setAnalysisError] = useState<string | null>(null);

  useEffect(() => {
    if (filePath && fileName) {
      fetchFileContent();
      fetchFileAnalysis();
    }
  }, [filePath, fileName]);

  const fetchFileContent = async () => {
    if (!filePath) return;
    
    setContentLoading(true);
    try {
      const response = await fetch(`http://127.0.0.1:8080/api/upload/content/${encodeURIComponent(filePath)}`, {
        headers: {
          'Accept': 'text/plain',
        },
      });
      
      if (!response.ok) {
        throw new Error(`Failed to fetch file content: ${response.status} ${response.statusText}`);
      }
      
      const content = await response.text();
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
        const errorData = await response.json().catch(() => null);
        const errorMessage = errorData?.message || `Failed to fetch analysis: ${response.status} ${response.statusText}`;
        throw new Error(errorMessage);
      }
      
      const analysis = await response.json();
      setFileAnalysis(analysis);
    } catch (error) {
      console.error('Error fetching file analysis:', error);
      setAnalysisError(error instanceof Error ? error.message : 'Failed to fetch analysis');
    } finally {
      setAnalysisLoading(false);
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

          <TabsContent value="contents" className="mt-4">
            {contentLoading ? (
              <div className="space-y-2">
                <Skeleton className="h-4 w-full" />
                <Skeleton className="h-4 w-[90%]" />
                <Skeleton className="h-4 w-[85%]" />
              </div>
            ) : fileContent ? (
              <ScrollArea className="h-[600px]">
                <SyntaxHighlighter
                  language={fileAnalysis?.language?.toLowerCase() || 'text'}
                  style={vscDarkPlus}
                  showLineNumbers
                  wrapLines
                >
                  {fileContent}
                </SyntaxHighlighter>
              </ScrollArea>
            ) : (
              <div className="text-center py-8 text-muted-foreground">
                <CodeIcon className="h-8 w-8 mx-auto mb-2 opacity-50" />
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
                    <p className="text-muted-foreground">{fileAnalysis.file_purpose}</p>
                  </div>
                  
                  <div className="text-sm text-muted-foreground">
                    Last updated: {new Date(fileAnalysis.analysis_time).toLocaleString()}
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

          <TabsContent value="dependencies" className="space-y-4">
            {analysisLoading ? (
              <div className="space-y-2">
                <Skeleton className="h-4 w-full" />
                <Skeleton className="h-4 w-[90%]" />
                <Skeleton className="h-4 w-[85%]" />
              </div>
            ) : analysisError ? (
              <div className="text-center py-8">
                <AlertCircleIcon className="h-8 w-8 mx-auto mb-2 text-destructive" />
                <p className="text-destructive mb-4">{analysisError}</p>
                <div className="space-y-2">
                  <Button 
                    variant="outline" 
                    className="w-full"
                    onClick={fetchFileAnalysis}
                  >
                    <RefreshCwIcon className="h-4 w-4 mr-2" />
                    Retry Analysis
                  </Button>
                  <p className="text-sm text-muted-foreground">
                    If the error persists, please try again in a few moments.
                  </p>
                </div>
              </div>
            ) : fileAnalysis ? (
              <ScrollArea className="h-[600px] pr-4">
                <div className="space-y-6">
                  <div>
                    <h3 className="text-lg font-semibold mb-4">Dependencies</h3>
                    {fileAnalysis.dependencies && fileAnalysis.dependencies.length > 0 ? (
                      <div className="grid gap-4">
                        {fileAnalysis.dependencies.map((dep, index) => (
                          <div key={index} className="flex items-start space-x-2">
                            <Badge variant="secondary" className="mt-1">Dependency</Badge>
                            <div>
                              <p className="text-sm">{dep}</p>
                            </div>
                          </div>
                        ))}
                      </div>
                    ) : (
                      <p className="text-muted-foreground">No dependencies found</p>
                    )}
                  </div>
                  
                  <Button 
                    variant="outline" 
                    className="w-full"
                    onClick={fetchFileAnalysis}
                  >
                    <RefreshCwIcon className="h-4 w-4 mr-2" />
                    Refresh Dependencies
                  </Button>
                </div>
              </ScrollArea>
            ) : (
              <div className="text-center py-8 text-muted-foreground">
                <BoxesIcon className="h-8 w-8 mx-auto mb-2 opacity-50" />
                <p>No dependencies available</p>
              </div>
            )}
          </TabsContent>
        </Tabs>
      </CardContent>
    </Card>
  );
}

// ... existing getLanguageFromFileName function ... 