import React, { useState, useEffect } from 'react';
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from '@/components/ui/card';
import { Tabs, TabsList, TabsTrigger, TabsContent } from '@/components/ui/tabs';
import { ScrollArea } from '@/components/ui/scroll-area';
import { Skeleton } from '@/components/ui/skeleton';
import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import { FileTextIcon, CodeIcon, BrainCogIcon, AlertCircleIcon, RefreshCwIcon } from 'lucide-react';
import { SyntaxHighlighter } from '@/components/ui/syntax-highlighter';
import { vscDarkPlus } from '@/lib/prism-themes';

interface FileAnalysis {
  language: string;
  file_purpose: string;
  dependencies: string[];
  analysis_time: string;
  contents: string;
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
      const response = await fetch(`http://127.0.0.1:8080/api/analysis/${encodeURIComponent(filePath)}`, {
        headers: {
          'Accept': 'application/json',
        },
      });
      
      if (!response.ok) {
        throw new Error(`Failed to fetch analysis: ${response.status} ${response.statusText}`);
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
          <TabsList className="grid w-full grid-cols-2">
            <TabsTrigger value="contents" className="flex items-center gap-2">
              <CodeIcon className="h-4 w-4" />
              Contents
            </TabsTrigger>
            <TabsTrigger value="analysis" className="flex items-center gap-2">
              <BrainCogIcon className="h-4 w-4" />
              AI Analysis
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
                    <p className="text-muted-foreground">{fileAnalysis.file_purpose}</p>
                  </div>
                  
                  <div>
                    <h3 className="text-lg font-semibold mb-2">Dependencies</h3>
                    {fileAnalysis.dependencies.length > 0 ? (
                      <div className="flex flex-wrap gap-2">
                        {fileAnalysis.dependencies.map((dep, index) => (
                          <Badge key={index} variant="secondary">{dep}</Badge>
                        ))}
                      </div>
                    ) : (
                      <p className="text-muted-foreground">No dependencies found</p>
                    )}
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
        </Tabs>
      </CardContent>
    </Card>
  );
}

// ... existing getLanguageFromFileName function ... 