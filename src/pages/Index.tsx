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

interface IndexProps {
  isAuthenticated: boolean;
  onLogin: () => void;
  onLogout: () => void;
}

const Index = ({ isAuthenticated, onLogin, onLogout }: IndexProps) => {
  const [hasUploadedFile, setHasUploadedFile] = useState(false);
  const [uploadedFileName, setUploadedFileName] = useState("");
  const [selectedFile, setSelectedFile] = useState<FileNode | null>(null);
  const [selectedFilePath, setSelectedFilePath] = useState("");
  const [loginDialogOpen, setLoginDialogOpen] = useState(false);
  
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
  };
  
  const handleUploadComplete = (response: any) => {
    setHasUploadedFile(true);
    const cleanFileName = response.filename.split('_').slice(1).join('_').replace('.zip', '');
    setUploadedFileName(cleanFileName);
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
