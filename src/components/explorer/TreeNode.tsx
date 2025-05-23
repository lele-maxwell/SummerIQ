
import { useState } from "react";
import { ChevronDownIcon, ChevronRightIcon, DownloadIcon } from "lucide-react";
import { cn } from "@/lib/utils";
import { FileNode } from "./types";
import { getFileIcon } from "./utils/fileIcons";

interface TreeNodeProps {
  node: FileNode;
  level: number;
  onSelect: (node: FileNode) => void;
  selectedPath: string;
  path: string;
}

export const TreeNode: React.FC<TreeNodeProps> = ({ node, level, onSelect, selectedPath, path }) => {
  const [isExpanded, setIsExpanded] = useState(level === 0);
  const currentPath = `${path}/${node.name}`;
  const isSelected = currentPath === selectedPath;
  
  const toggleExpand = (e: React.MouseEvent) => {
    e.stopPropagation();
    setIsExpanded(!isExpanded);
  };

  const handleSelect = () => {
    onSelect(node);
  };

  const handleDownload = (e: React.MouseEvent) => {
    e.stopPropagation();
    if (node.type === "file") {
      downloadFile(node);
    }
  };

  const downloadFile = (file: FileNode) => {
    // Create a dummy content for demo purposes
    // In a real app, you would fetch the actual file content
    const content = `This is the content of ${file.name}`;
    
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
  };

  return (
    <div>
      <div 
        className={cn(
          "flex items-center py-1 px-2 rounded-md cursor-pointer transition-colors",
          isSelected ? "bg-secondary text-primary" : "hover:bg-secondary/50"
        )}
        style={{ paddingLeft: `${level * 12 + 8}px` }}
        onClick={handleSelect}
      >
        {node.type === "folder" && (
          <span className="mr-1 cursor-pointer" onClick={toggleExpand}>
            {isExpanded ? <ChevronDownIcon className="h-4 w-4" /> : <ChevronRightIcon className="h-4 w-4" />}
          </span>
        )}
        {node.type !== "folder" && <span className="w-5" />}
        {getFileIcon(node.name, node.type, node.extension)}
        <span className="ml-2 text-sm truncate">{node.name}</span>
        
        {node.type === "file" && (
          <span 
            className="ml-auto cursor-pointer text-muted-foreground hover:text-primary"
            onClick={handleDownload}
            title="Download file"
          >
            <DownloadIcon className="h-4 w-4" />
          </span>
        )}
      </div>
      
      {isExpanded && node.children && (
        <div>
          {node.children.map((childNode) => (
            <TreeNode
              key={childNode.name}
              node={childNode}
              level={level + 1}
              onSelect={onSelect}
              selectedPath={selectedPath}
              path={currentPath}
            />
          ))}
        </div>
      )}
    </div>
  );
};
