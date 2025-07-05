import { useState, useEffect } from "react";
import { Header } from "@/components/layout/Header";
import { FileExplorer } from "@/components/explorer/FileExplorer";
import { AIAnalysis } from "@/components/analysis/AIAnalysis";
import { ChatInterface } from "@/components/chat/ChatInterface";
import { ResizableHandle, ResizablePanel, ResizablePanelGroup } from "@/components/ui/resizable";
import { FileNode } from "@/components/explorer/types";
import { useNavigate } from "react-router-dom";
import { BookOpenIcon } from "lucide-react";

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
  const navigate = useNavigate();

  useEffect(() => {
    console.log('Dashboard mounted');
    // Only restore from localStorage if authenticated
    if (!isAuthenticated) {
      setUploadedFileName("");
      setFileStructure(null);
      setIsLoading(false);
      return;
    }
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
  }, [isAuthenticated]);

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
          onLogin={() => {}} 
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
    // Redirect to upload page if no project data
    navigate('/', { replace: true });
    return null;
  }

  return (
    <div className="min-h-screen flex flex-col bg-background">
      <Header 
        isAuthenticated={isAuthenticated} 
        onLogin={() => {}} 
        onLogout={onLogout} 
      />
      
      <div className="flex-grow container mx-auto py-6">
        <div className="flex justify-end mb-4">
          <button
            className="inline-flex items-center gap-2 px-5 py-2 rounded-lg bg-gradient-to-r from-zipmind-400 to-zipmind-600 text-white font-semibold shadow-lg hover:scale-105 transition-transform duration-200"
            onClick={() => navigate('/architecture')}
          >
            <BookOpenIcon className="w-5 h-5" />
            Project Architecture
          </button>
        </div>
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
              <ChatInterface 
                projectName={uploadedFileName} 
                selectedFileName={selectedFile?.name}
                selectedFilePath={selectedFilePath}
              />
            </div>
          </ResizablePanel>
        </ResizablePanelGroup>
      </div>
    </div>
  );
};

export default Dashboard; 