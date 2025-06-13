import { useState, useEffect } from "react";
import { Header } from "@/components/layout/Header";
import { FileExplorer } from "@/components/explorer/FileExplorer";
import { AIAnalysis } from "@/components/analysis/AIAnalysis";
import { ChatInterface } from "@/components/chat/ChatInterface";
import { ResizableHandle, ResizablePanel, ResizablePanelGroup } from "@/components/ui/resizable";
import { FileNode } from "@/components/explorer/types";

interface DashboardProps {
  isAuthenticated: boolean;
  onLogout: () => void;
}

const Dashboard = ({ isAuthenticated, onLogout }: DashboardProps) => {
  const [selectedFile, setSelectedFile] = useState<FileNode | null>(null);
  const [selectedFilePath, setSelectedFilePath] = useState("");
  const [uploadedFileName, setUploadedFileName] = useState("");
  const [fileStructure, setFileStructure] = useState<FileNode | null>(null);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    console.log('Dashboard mounted');
    // Get the file information from localStorage
    const storedFileName = localStorage.getItem('uploadedFileName');
    const storedFileStructure = localStorage.getItem('fileStructure');
    
    console.log('Stored file name:', storedFileName);
    console.log('Stored file structure exists:', !!storedFileStructure);
    
    if (storedFileName) {
      console.log('Setting uploaded file name:', storedFileName);
      setUploadedFileName(storedFileName);
    }
    
    if (storedFileStructure) {
      try {
        const parsedStructure = JSON.parse(storedFileStructure);
        console.log('Parsed file structure:', parsedStructure);
        
        // Ensure the structure has the correct name
        if (parsedStructure.name === '') {
          parsedStructure.name = storedFileName || 'Project';
        }
        
        setFileStructure(parsedStructure);
      } catch (error) {
        console.error('Error parsing file structure:', error);
      }
    }
    
    setIsLoading(false);
  }, []);

  const handleFileSelect = (file: FileNode, path: string) => {
    console.log('File selected:', file);
    console.log('File path:', path);
    setSelectedFile(file);
    setSelectedFilePath(path);
  };

  if (isLoading) {
    return (
      <div className="min-h-screen flex flex-col bg-background">
        <Header 
          isAuthenticated={isAuthenticated} 
          onLogout={onLogout} 
        />
        <div className="flex-grow container mx-auto py-6">
          <div className="text-center">
            <h1 className="text-2xl font-bold mb-4">Loading...</h1>
            <p className="text-muted-foreground">Please wait while we load your project data.</p>
          </div>
        </div>
      </div>
    );
  }

  if (!uploadedFileName || !fileStructure) {
    console.log('Missing required data:', { uploadedFileName, fileStructure });
    return (
      <div className="min-h-screen flex flex-col bg-background">
        <Header 
          isAuthenticated={isAuthenticated} 
          onLogout={onLogout} 
        />
        <div className="flex-grow container mx-auto py-6">
          <div className="text-center">
            <h1 className="text-2xl font-bold mb-4">No Project Data</h1>
            <p className="text-muted-foreground">Please upload a project first.</p>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="min-h-screen flex flex-col bg-background">
      <Header 
        isAuthenticated={isAuthenticated} 
        onLogout={onLogout} 
      />
      
      <div className="flex-grow container mx-auto py-6">
        <div className="mb-6">
          <h1 className="text-2xl font-bold">Project: {uploadedFileName}</h1>
          <p className="text-muted-foreground">
            Uploaded {new Date().toLocaleDateString()} · Click on files to analyze
          </p>
        </div>
        
        <ResizablePanelGroup 
          direction="horizontal" 
          className="min-h-[calc(100vh-200px)] max-h-[calc(100vh-200px)] rounded-lg border"
        >
          <ResizablePanel defaultSize={20} minSize={15}>
            <div className="h-full">
              <FileExplorer fileStructure={fileStructure} onFileSelect={handleFileSelect} />
            </div>
          </ResizablePanel>
          
          <ResizableHandle withHandle />
          
          <ResizablePanel defaultSize={50} minSize={30}>
            <div className="h-full overflow-auto">
              <AIAnalysis fileName={selectedFile?.name} filePath={selectedFilePath} />
            </div>
          </ResizablePanel>
          
          <ResizableHandle withHandle />
          
          <ResizablePanel defaultSize={30} minSize={20}>
            <div className="h-full">
              <ChatInterface projectName={uploadedFileName} />
            </div>
          </ResizablePanel>
        </ResizablePanelGroup>
      </div>
    </div>
  );
};

export default Dashboard; 