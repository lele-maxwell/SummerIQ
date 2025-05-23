
import { FileNode } from "../types";

export const downloadFile = (file: FileNode, fileContent?: string) => {
  // Use provided content or generate dummy content for demo
  const content = fileContent || `This is the content of ${file.name}`;
  
  // Create a blob from the content
  const blob = new Blob([content], { type: 'text/plain' });
  
  // Create an anchor element and set properties for download
  const link = document.createElement('a');
  link.href = URL.createObjectURL(blob);
  link.download = file.name;
  
  // Append to the document, trigger click and then remove
  document.body.appendChild(link);
  link.click();
  document.body.removeChild(link);
  
  // Clean up the object URL
  setTimeout(() => {
    URL.revokeObjectURL(link.href);
  }, 100);
};

// For downloading an entire folder as a zip file
// This is a placeholder - in a real app you would need to use a library like JSZip
export const downloadFolder = (folder: FileNode) => {
  alert(`Downloading folder "${folder.name}" is not implemented in this demo.\nIn a real application, this would create a zip file of all contents.`);
};
