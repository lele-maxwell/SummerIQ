import { useState, useEffect } from "react";
import { ScrollArea } from "@/components/ui/scroll-area";
import { DownloadIcon } from "lucide-react";
import { Button } from "@/components/ui/button";
import { FileNode } from "./types";
import { TreeNode } from "./TreeNode";
import { downloadFolder } from "./utils/fileUtils";

interface FileExplorerProps {
  fileStructure: FileNode | null;
  onFileSelect: (file: FileNode, path: string) => void;
}

export const FileExplorer = ({ fileStructure, onFileSelect }: FileExplorerProps) => {
  const [selectedPath, setSelectedPath] = useState("");
  
  useEffect(() => {
    console.log('FileExplorer received structure:', JSON.stringify(fileStructure, null, 2));
    if (fileStructure?.children) {
      console.log('Number of root items:', fileStructure.children.length);
      fileStructure.children.forEach((child, index) => {
        console.log(`Root item ${index + 1}:`, child);
      });
    }
  }, [fileStructure]);
  
  const handleNodeSelect = (node: FileNode) => {
    if (node.is_dir) {
      return;
    }

    // Find the path by traversing the tree
    const findPath = (nodes: FileNode[], targetName: string, currentPath: string): string | null => {
      for (const node of nodes) {
        const newPath = `${currentPath}/${node.name}`;
        if (node.name === targetName) return newPath;
        if (node.children) {
          const foundPath = findPath(node.children, targetName, newPath);
          if (foundPath) return foundPath;
        }
      }
      return null;
    };
    
    const path = findPath([fileStructure], node.name, "");
    if (path) {
      console.log('Selected path:', path);
      setSelectedPath(path);
      onFileSelect(node, path);
    }
  };

  const handleDownloadAll = () => {
    if (fileStructure) {
      downloadFolder(fileStructure);
    }
  };

  if (!fileStructure) {
    return null;
  }

  return (
    <div className="border rounded-md h-full">
      <div className="bg-green-900 text-green-50 p-2 border-b text-sm font-medium flex justify-between items-center">
        <span>Project Files</span>
        <Button 
          variant="ghost" 
          size="sm" 
          className="h-8 px-2 text-green-50 hover:text-white hover:bg-green-800" 
          onClick={handleDownloadAll}
          title="Download all files"
        >
          <DownloadIcon className="h-4 w-4" />
        </Button>
      </div>
      <ScrollArea className="h-[calc(100%-40px)]">
        <div className="p-2">
          <TreeNode 
            node={fileStructure} 
            level={0} 
            onSelect={handleNodeSelect}
            selectedPath={selectedPath}
            path=""
          />
        </div>
      </ScrollArea>
    </div>
  );
}
