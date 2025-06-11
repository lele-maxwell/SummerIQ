import { useState } from "react";
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

  const handleFileSelect = (file: FileNode, path: string) => {
    setSelectedFile(file);
    setSelectedFilePath(path);
  };

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
              <FileExplorer onFileSelect={handleFileSelect} />
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