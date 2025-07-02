import { useState, useRef } from "react";
import { Button } from "@/components/ui/button";
import { UploadCloudIcon, FileIcon, Loader2Icon, CheckIcon, XIcon, FolderIcon } from "lucide-react";
import { Progress } from "@/components/ui/progress";
import { API, UploadResponse } from "@/types/api";
import { getApiUrl } from "@/lib/api";
import { uploadProject } from "@/api/upload";
import { FileNode } from "@/components/explorer/types";

interface FileUploadProps {
  onUploadComplete: (response: UploadResponse) => void;
}

export function FileUpload({ onUploadComplete }: FileUploadProps) {
  const fileInputRef = useRef<HTMLInputElement>(null);
  const [file, setFile] = useState<File | null>(null);
  const [dragOver, setDragOver] = useState(false);
  const [uploading, setUploading] = useState(false);
  const [progress, setProgress] = useState(0);
  const [error, setError] = useState<string | null>(null);
  const [uploadResult, setUploadResult] = useState<UploadResponse | null>(null);

  const handleDrop = (e: React.DragEvent) => {
    e.preventDefault();
    setDragOver(false);
    
    const droppedFile = e.dataTransfer.files[0];
    if (droppedFile && (droppedFile.name.endsWith('.zip') || droppedFile.name.endsWith('.sip'))) {
      setFile(droppedFile);
    } else {
      setError('Please upload a ZIP or SIP file');
    }
  };

  const handleDragOver = (e: React.DragEvent) => {
    e.preventDefault();
    setDragOver(true);
  };

  const handleDragLeave = (e: React.DragEvent) => {
    e.preventDefault();
    setDragOver(false);
  };

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const selectedFile = e.target.files?.[0];
    if (selectedFile && (selectedFile.name.endsWith('.zip') || selectedFile.name.endsWith('.sip'))) {
      setFile(selectedFile);
      setError(null);
    } else {
      setError('Please upload a ZIP or SIP file');
    }
  };

  const handleUpload = async () => {
    if (!file) return;

    setUploading(true);
    setError(null);
    setProgress(0);
    setUploadResult(null);

    try {
      const token = localStorage.getItem('token');
      if (!token) {
        setError('Please log in to upload files');
        return;
      }

      const result = await uploadProject(file, token);
      console.log('Upload response:', result);
      
      if (result && result.upload) {
        console.log('Upload data:', result.upload);
        setUploadResult(result);
        onUploadComplete(result);
        
        // Store upload data in localStorage
        localStorage.setItem('uploadData', JSON.stringify({
          filename: result.filename,
          upload: result.upload,
          extracted_files: result.extracted_files
        }));
        // Store uploaded project name (after first underscore, without extension) for documentation API
        // Example: '7c3e8258-849f-4d3f-b412-351a2089eec3_business-platform(1).zip' -> 'business-platform(1)'
        const projectName = result.filename.split('_').slice(1).join('_').replace(/\.(zip|sip)$/i, '');
        localStorage.setItem('uploadedFileName', projectName);
      } else {
        console.error('Unexpected response format:', result);
        setError('Received unexpected response format from server');
      }
    } catch (err) {
      console.error('Upload error:', err);
      setError(err instanceof Error ? err.message : 'Upload failed');
    } finally {
      setUploading(false);
      setProgress(0);
    }
  };

  const handleRemove = () => {
    setFile(null);
    setError(null);
    if (fileInputRef.current) {
      fileInputRef.current.value = '';
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
              accept=".zip,.sip"
              className="hidden"
            />
          </>
        ) : (
          <div className="w-full">
            <div className="flex items-center justify-between mb-4">
              <div className="flex items-center space-x-2">
                <FileIcon className="h-5 w-5 text-muted-foreground" />
                <span className="font-medium">{file.name}</span>
              </div>
              <Button
                variant="ghost"
                size="icon"
                onClick={handleRemove}
                disabled={uploading}
              >
                <XIcon className="h-4 w-4" />
              </Button>
            </div>
            
            {error && (
              <div className="mb-4 p-3 bg-destructive/10 text-destructive text-sm rounded-md">
                {error}
              </div>
            )}

            {uploading && (
              <div className="space-y-2">
                <Progress value={progress} className="w-full" />
                <p className="text-sm text-muted-foreground text-center">
                  Uploading... {progress}%
                </p>
              </div>
            )}

            <Button
              onClick={handleUpload}
              disabled={uploading}
              className="w-full"
            >
              {uploading ? (
                <>
                  <Loader2Icon className="mr-2 h-4 w-4 animate-spin" />
                  Uploading...
                </>
              ) : (
                <>
                  <UploadCloudIcon className="mr-2 h-4 w-4" />
                  Upload Project
                </>
              )}
            </Button>

            {uploadResult && (
              <div className="mt-4 p-4 bg-muted rounded-lg">
                <h4 className="font-medium mb-2 flex items-center">
                  <FolderIcon className="h-4 w-4 mr-2" />
                  Upload Results
                </h4>
                <div className="space-y-2">
                  {uploadResult.upload?.extracted_files && (
                    <>
                      <h5 className="font-medium text-sm mt-2">Extracted Files:</h5>
                      <ul className="space-y-1">
                        {uploadResult.upload.extracted_files.map((file: FileNode, index: number) => (
                          <li key={index} className="text-sm text-muted-foreground">
                            {file.name}
                          </li>
                        ))}
                      </ul>
                    </>
                  )}
                </div>
              </div>
            )}
          </div>
        )}
      </div>
      <p className="text-xs text-muted-foreground mt-2 text-center">
        Maximum file size: 50MB. Supported formats: .zip, .sip
      </p>
    </div>
  );
}
