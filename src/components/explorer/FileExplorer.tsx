
import { useState } from "react";
import { ScrollArea } from "@/components/ui/scroll-area";
import { FileNode } from "./types";
import { TreeNode } from "./TreeNode";
import { demoFileStructure } from "./mockData";

interface FileExplorerProps {
  fileStructure?: FileNode;
  onFileSelect: (file: FileNode, path: string) => void;
}

export function FileExplorer({ fileStructure = demoFileStructure, onFileSelect }: FileExplorerProps) {
  const [selectedPath, setSelectedPath] = useState("");
  
  const handleNodeSelect = (node: FileNode) => {
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
      setSelectedPath(path);
      onFileSelect(node, path);
    }
  };

  return (
    <div className="border rounded-md h-full">
      <div className="bg-muted p-2 border-b text-sm font-medium">
        Project Files
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
