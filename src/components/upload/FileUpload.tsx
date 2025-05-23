
import { useState, useRef } from "react";
import { Button } from "@/components/ui/button";
import { UploadCloudIcon, FileIcon, Loader2Icon, CheckIcon, XIcon } from "lucide-react";
import { useToast } from "@/components/ui/use-toast";
import { Progress } from "@/components/ui/progress";

interface FileUploadProps {
  onUploadComplete: (filename: string) => void;
}

export function FileUpload({ onUploadComplete }: FileUploadProps) {
  const { toast } = useToast();
  const fileInputRef = useRef<HTMLInputElement>(null);
  const [file, setFile] = useState<File | null>(null);
  const [uploading, setUploading] = useState(false);
  const [uploadProgress, setUploadProgress] = useState(0);
  const [dragOver, setDragOver] = useState(false);

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const selectedFile = e.target.files?.[0];
    if (selectedFile && selectedFile.name.endsWith(".zip")) {
      setFile(selectedFile);
    } else if (selectedFile) {
      toast({
        variant: "destructive",
        title: "Invalid file",
        description: "Please upload a ZIP file.",
      });
    }
  };

  const handleDrop = (e: React.DragEvent<HTMLDivElement>) => {
    e.preventDefault();
    setDragOver(false);
    
    const droppedFile = e.dataTransfer.files[0];
    if (droppedFile && droppedFile.name.endsWith(".zip")) {
      setFile(droppedFile);
    } else if (droppedFile) {
      toast({
        variant: "destructive",
        title: "Invalid file",
        description: "Please upload a ZIP file.",
      });
    }
  };
  
  const handleDragOver = (e: React.DragEvent<HTMLDivElement>) => {
    e.preventDefault();
    setDragOver(true);
  };
  
  const handleDragLeave = () => {
    setDragOver(false);
  };

  const handleUpload = () => {
    if (!file) return;
    
    setUploading(true);
    
    // Simulate file upload with progress
    let progress = 0;
    const interval = setInterval(() => {
      progress += Math.random() * 10;
      if (progress > 100) progress = 100;
      setUploadProgress(Math.floor(progress));
      
      if (progress === 100) {
        clearInterval(interval);
        setTimeout(() => {
          setUploading(false);
          toast({
            title: "Upload successful",
            description: `${file.name} has been uploaded.`,
          });
          onUploadComplete(file.name);
        }, 500);
      }
    }, 300);
  };

  const resetUpload = () => {
    setFile(null);
    setUploadProgress(0);
    if (fileInputRef.current) {
      fileInputRef.current.value = "";
    }
  };

  return (
    <div className="w-full max-w-3xl mx-auto">
      <div 
        className={`border-2 border-dashed rounded-lg p-6 flex flex-col items-center justify-center transition-all ${
          dragOver ? "border-primary bg-primary/5" : "border-border"
        }`}
        onDrop={handleDrop}
        onDragOver={handleDragOver}
        onDragLeave={handleDragLeave}
      >
        {!file ? (
          <>
            <UploadCloudIcon className="h-12 w-12 text-muted-foreground mb-4" />
            <h3 className="font-medium text-lg mb-1">Upload Project ZIP</h3>
            <p className="text-muted-foreground text-sm mb-4 text-center">
              Drag and drop your ZIP file here, or click to select
            </p>
            <Button 
              onClick={() => fileInputRef.current?.click()}
              variant="outline" 
              className="mt-2"
            >
              Select ZIP File
            </Button>
            <input
              type="file"
              ref={fileInputRef}
              onChange={handleFileChange}
              accept=".zip"
              className="hidden"
            />
          </>
        ) : (
          <div className="w-full">
            <div className="flex items-center justify-between mb-4">
              <div className="flex items-center">
                <FileIcon className="h-6 w-6 mr-2 text-muted-foreground" />
                <div>
                  <p className="font-medium">{file.name}</p>
                  <p className="text-xs text-muted-foreground">
                    {(file.size / (1024 * 1024)).toFixed(2)} MB
                  </p>
                </div>
              </div>
              
              <button 
                onClick={resetUpload} 
                className="p-1 hover:bg-secondary rounded-full"
                disabled={uploading}
              >
                <XIcon className="h-4 w-4" />
              </button>
            </div>
            
            {uploading ? (
              <div className="space-y-2">
                <Progress value={uploadProgress} className="h-2" />
                <div className="flex justify-between text-xs text-muted-foreground">
                  <span>Uploading...</span>
                  <span>{uploadProgress}%</span>
                </div>
              </div>
            ) : (
              <Button onClick={handleUpload} className="w-full mt-2">
                Upload and Analyze
              </Button>
            )}
          </div>
        )}
      </div>
      <p className="text-xs text-muted-foreground mt-2 text-center">
        Maximum file size: 50MB. Supported format: .zip
      </p>
    </div>
  );
}
