import { useState } from "react";
import { Button } from "@/components/ui/button";
import { Dialog, DialogContent } from "@/components/ui/dialog";
import { Header } from "@/components/layout/Header";
import { AuthForm } from "@/components/auth/AuthForm";
import { FileUpload } from "@/components/upload/FileUpload";
import { FileExplorer } from "@/components/explorer/FileExplorer";
import { AIAnalysis } from "@/components/analysis/AIAnalysis";
import { ChatInterface } from "@/components/chat/ChatInterface";
import { ResizableHandle, ResizablePanel, ResizablePanelGroup } from "@/components/ui/resizable";
import { BrainCogIcon, UploadCloudIcon, LayoutPanelLeftIcon, MessageSquareTextIcon } from "lucide-react";
import { FileNode } from "@/components/explorer/types";
import { UploadResponse } from "@/types/api";
import { useNavigate } from "react-router-dom";

interface IndexProps {
  isAuthenticated: boolean;
  onLogin: () => void;
  onLogout: () => void;
}

interface FileObject {
  name: string;
  path: string;
  is_dir: boolean;
  children?: FileObject[] | null;
}

const Index = ({ isAuthenticated, onLogin, onLogout }: IndexProps) => {
  const navigate = useNavigate();
  const [hasUploadedFile, setHasUploadedFile] = useState(false);
  const [uploadedFileName, setUploadedFileName] = useState("");
  const [selectedFile, setSelectedFile] = useState<FileNode | null>(null);
  const [selectedFilePath, setSelectedFilePath] = useState("");
  const [loginDialogOpen, setLoginDialogOpen] = useState(false);
  const [fileStructure, setFileStructure] = useState<FileNode | null>(null);
  
  const handleLogin = () => {
    onLogin();
    setLoginDialogOpen(false);
  };
  
  const handleLogout = () => {
    onLogout();
    setHasUploadedFile(false);
    setUploadedFileName("");
    setSelectedFile(null);
    setSelectedFilePath("");
    setFileStructure(null);
  };
  
  const convertToFileStructure = (extractedFiles: FileObject[]): FileNode => {
    console.log('Converting extracted files:', extractedFiles);
    
    const root: FileNode = {
      name: uploadedFileName,
      type: "folder",
      children: []
    };

    const processFile = (file: FileObject, parent: FileNode) => {
      const node: FileNode = {
        name: file.name,
        type: file.is_dir ? "folder" : "file",
        extension: !file.is_dir ? file.name.split('.').pop() : undefined,
        children: file.is_dir ? [] : undefined
      };

      if (file.is_dir && file.children) {
        file.children.forEach(child => processFile(child, node));
      }

      parent.children = parent.children || [];
      parent.children.push(node);
    };

    // Process each file/directory
    extractedFiles.forEach(file => {
      processFile(file, root);
    });

    // Sort children: folders first, then files, both alphabetically
    const sortNode = (node: FileNode) => {
      if (node.children) {
        node.children.sort((a, b) => {
          // First sort by type (folders before files)
          if (a.type !== b.type) {
            return a.type === "folder" ? -1 : 1;
          }
          // Then sort alphabetically
          return a.name.localeCompare(b.name);
        });
        
        // Recursively sort children
        node.children.forEach(sortNode);
      }
    };
    
    sortNode(root);
    console.log('Final file structure:', JSON.stringify(root, null, 2));
    return root;
  };
  
  const handleUploadComplete = (response: UploadResponse) => {
    console.log('Upload response:', response);
    setHasUploadedFile(true);
    const cleanFileName = response.filename.split('_').slice(1).join('_').replace('.zip', '');
    console.log('Clean file name:', cleanFileName);
    setUploadedFileName(cleanFileName);
    
    if (response.upload?.extracted_files) {
      console.log('Raw extracted files:', response.upload.extracted_files);
      console.log('Number of files:', response.upload.extracted_files.length);
      
      // Log each file path separately
      response.upload.extracted_files.forEach((file, index) => {
        console.log(`File ${index + 1}:`, file);
      });
      
      // Filter out any empty paths and normalize the paths
      const validFiles = response.upload.extracted_files
        .filter(file => file && file.path && file.path.trim() !== '')
        .map(file => ({
          ...file,
          path: file.path.replace(/^\/+/, '').replace(/\/+$/, '')
        }));
      
      console.log('Valid files:', validFiles);
      
      const structure = convertToFileStructure(validFiles);
      // Ensure the root node has the correct name
      structure.name = cleanFileName;
      console.log('File structure created:', JSON.stringify(structure, null, 2));
      setFileStructure(structure);
      
      // Store file information in localStorage
      try {
        localStorage.setItem('uploadedFileName', cleanFileName);
        localStorage.setItem('fileStructure', JSON.stringify(structure));
        console.log('Data stored in localStorage');
        
        // Verify the stored data
        const storedStructure = localStorage.getItem('fileStructure');
        console.log('Stored structure:', storedStructure);
      } catch (error) {
        console.error('Error storing data in localStorage:', error);
      }
      
      // Navigate to dashboard after successful upload
      console.log('Navigating to dashboard...');
      navigate('/dashboard', { replace: true });
    } else {
      console.error('No extracted files in response:', response);
    }
  };
  
  const handleFileSelect = (file: FileNode, path: string) => {
    setSelectedFile(file);
    setSelectedFilePath(path);
  };

  return (
    <div className="min-h-screen flex flex-col bg-background">
      <Header 
        isAuthenticated={isAuthenticated} 
        onLogin={() => setLoginDialogOpen(true)} 
        onLogout={handleLogout} 
      />

      <Dialog open={loginDialogOpen} onOpenChange={setLoginDialogOpen}>
        <DialogContent className="sm:max-w-md">
          <AuthForm onSuccess={handleLogin} />
        </DialogContent>
      </Dialog>
      
      {!isAuthenticated ? (
        <div className="flex-grow flex flex-col items-center justify-center p-6">
          <HeroSection onLoginClick={() => setLoginDialogOpen(true)} />
        </div>
      ) : !hasUploadedFile ? (
        <div className="container mx-auto py-12">
          <h1 className="text-3xl font-bold text-center mb-2">Upload Your Project</h1>
          <p className="text-muted-foreground text-center mb-8">
            Upload a ZIP file containing your project to start analyzing it
          </p>
          <FileUpload onUploadComplete={handleUploadComplete} />
        </div>
      ) : (
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
      )}
    </div>
  );
};

const HeroSection = ({ onLoginClick }: { onLoginClick: () => void }) => {
  return (
    <>
      <div className="w-full max-w-5xl mx-auto text-center space-y-6">
        <div className="inline-flex items-center justify-center p-2 bg-secondary rounded-full mb-4">
          <BrainCogIcon className="h-8 w-8 text-zipmind-400" />
        </div>
        <h1 className="text-4xl md:text-6xl font-bold leading-tight">
          Understand code projects with 
          <span className="bg-gradient-to-r from-zipmind-400 to-zipmind-600 text-transparent bg-clip-text"> AI-powered insights</span>
        </h1>
        <p className="text-xl text-muted-foreground max-w-3xl mx-auto">
          Upload a zipped codebase and let ZipMind analyze it. Get detailed summaries, understand structure, 
          and chat with your code to find answers quickly.
        </p>
        
        <div className="flex flex-wrap gap-4 justify-center pt-4">
          <Button size="lg" className="text-base" onClick={onLoginClick}>Get Started</Button>
          <Button size="lg" variant="outline" className="text-base">Learn More</Button>
        </div>
      </div>
      
      <div className="w-full mt-16">
        <div className="grid grid-cols-1 md:grid-cols-3 gap-8 max-w-5xl mx-auto">
          <FeatureCard
            icon={<UploadCloudIcon className="h-8 w-8 text-zipmind-400" />}
            title="Upload Projects"
            description="Simply drag & drop your zipped project files for instant analysis."
          />
          <FeatureCard
            icon={<LayoutPanelLeftIcon className="h-8 w-8 text-zipmind-400" />}
            title="Get Insights"
            description="Receive detailed summaries and structure analysis of each file."
          />
          <FeatureCard
            icon={<MessageSquareTextIcon className="h-8 w-8 text-zipmind-400" />}
            title="Ask Questions"
            description="Chat with your code to quickly find answers about any file or function."
          />
        </div>
      </div>
    </>
  );
};

const FeatureCard = ({ 
  icon, 
  title, 
  description 
}: { 
  icon: React.ReactNode; 
  title: string; 
  description: string;
}) => {
  return (
    <div className="bg-card border rounded-xl p-6">
      <div className="bg-secondary inline-flex rounded-lg p-3 mb-4">{icon}</div>
      <h3 className="text-xl font-semibold mb-2">{title}</h3>
      <p className="text-muted-foreground">{description}</p>
    </div>
  );
};

export default Index;
