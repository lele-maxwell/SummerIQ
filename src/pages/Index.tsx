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
import { BrainCogIcon, UploadCloudIcon, LayoutPanelLeftIcon, MessageSquareTextIcon, BookOpenIcon, GraduationCapIcon, CodeIcon } from "lucide-react";
import { FileNode } from "@/components/explorer/types";
import { UploadResponse } from "@/types/api";
import { useNavigate } from "react-router-dom";
import { motion } from "framer-motion";

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
  
  const handleUploadComplete = (response: UploadResponse) => {
    console.log('Upload response:', response);
    setHasUploadedFile(true);
    const cleanFileName = response.filename.split('_').slice(1).join('_').replace('.zip', '');
    console.log('Clean file name:', cleanFileName);
    setUploadedFileName(cleanFileName);

    // Use the backend's FileNode tree directly
    const fileNodes = response.upload?.extracted_files || response.extracted_files;
    if (fileNodes && Array.isArray(fileNodes)) {
      // Wrap in a root node for the explorer
      const root: FileNode = {
        name: cleanFileName,
        path: cleanFileName,
        is_dir: true,
        children: fileNodes as any // FileNode[]
      };
      setFileStructure(root);
      localStorage.setItem('uploadedFileName', cleanFileName);
      localStorage.setItem('fileStructure', JSON.stringify(root));
      console.log('File structure created:', JSON.stringify(root, null, 2));
      navigate('/dashboard', { replace: true });
    } else {
      console.error('No extracted file nodes in response:', response);
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
          
          {fileStructure && (
            (() => { console.log("Final fileStructure for explorer:", JSON.stringify(fileStructure, null, 2)); return null; })()
          )}
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
      <motion.div 
        className="w-full max-w-5xl mx-auto text-center space-y-6"
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ duration: 0.5 }}
      >
        <motion.div 
          className="inline-flex items-center justify-center p-2 bg-secondary rounded-full mb-4"
          initial={{ scale: 0 }}
          animate={{ scale: 1 }}
          transition={{ delay: 0.2, type: "spring", stiffness: 200 }}
        >
          <BrainCogIcon className="h-8 w-8 text-zipmind-400" />
        </motion.div>
        <motion.h1 
          className="text-4xl md:text-6xl font-bold leading-tight"
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.3, duration: 0.5 }}
        >
          Master project architecture with 
          <span className="bg-gradient-to-r from-zipmind-400 to-zipmind-600 text-transparent bg-clip-text"> AI-powered learning</span>
        </motion.h1>
        <motion.p 
          className="text-xl text-muted-foreground max-w-3xl mx-auto"
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.4, duration: 0.5 }}
        >
          Upload a project and let ZipMind guide you through its architecture. Learn modern technologies, 
          understand best practices, and contribute with confidence.
        </motion.p>
        
        <motion.div 
          className="flex flex-wrap gap-4 justify-center pt-4"
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.5, duration: 0.5 }}
        >
          <Button size="lg" className="text-base bg-zipmind-500 hover:bg-zipmind-600" onClick={onLoginClick}>
            Get Started
          </Button>
          <Button size="lg" variant="outline" className="text-base">Learn More</Button>
        </motion.div>
      </motion.div>
      
      <motion.div 
        className="w-full mt-16"
        initial={{ opacity: 0, y: 40 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ delay: 0.6, duration: 0.5 }}
      >
        <div className="grid grid-cols-1 md:grid-cols-3 gap-8 max-w-5xl mx-auto">
          <motion.div
            initial={{ opacity: 0, x: -20 }}
            animate={{ opacity: 1, x: 0 }}
            transition={{ delay: 0.7, duration: 0.5 }}
          >
            <FeatureCard
              icon={<BookOpenIcon className="h-8 w-8 text-zipmind-400" />}
              title="Project Understanding"
              description="Learn how different parts of a project connect and work together through AI-generated documentation."
            />
          </motion.div>
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.8, duration: 0.5 }}
          >
            <FeatureCard
              icon={<GraduationCapIcon className="h-8 w-8 text-zipmind-400" />}
              title="Learning Platform"
              description="Discover modern technologies and best practices used in real-world projects through interactive learning."
            />
          </motion.div>
          <motion.div
            initial={{ opacity: 0, x: 20 }}
            animate={{ opacity: 1, x: 0 }}
            transition={{ delay: 0.9, duration: 0.5 }}
          >
            <FeatureCard
              icon={<CodeIcon className="h-8 w-8 text-zipmind-400" />}
              title="Contribute Confidently"
              description="Understand where and how to add your code with AI-powered guidance and best practice suggestions."
            />
          </motion.div>
        </div>
      </motion.div>
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
    <motion.div 
      className="bg-card border rounded-xl p-6 h-full hover:shadow-lg transition-shadow duration-300"
      whileHover={{ scale: 1.02 }}
      transition={{ type: "spring", stiffness: 300 }}
    >
      <div className="bg-secondary inline-flex rounded-lg p-3 mb-4">{icon}</div>
      <h3 className="text-xl font-semibold mb-2">{title}</h3>
      <p className="text-muted-foreground">{description}</p>
    </motion.div>
  );
};

export default Index;
