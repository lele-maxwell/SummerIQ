
import { useState } from "react";
import { ChevronDownIcon, ChevronRightIcon, FileIcon, FileTextIcon, FolderIcon, CodeIcon, PackageIcon, DatabaseIcon, ImageIcon } from "lucide-react";
import { cn } from "@/lib/utils";
import { ScrollArea } from "@/components/ui/scroll-area";

// Define FileNode type before using it
type FileNode = {
  name: string;
  type: "file" | "folder";
  extension?: string;
  children?: FileNode[];
};

// Mock file structure for demo purposes - with proper type annotations
const demoFileStructure: FileNode = {
  name: "project-name",
  type: "folder",
  children: [
    {
      name: "src",
      type: "folder",
      children: [
        {
          name: "main.rs",
          type: "file",
          extension: "rs"
        },
        {
          name: "routes",
          type: "folder",
          children: [
            {
              name: "auth.rs",
              type: "file",
              extension: "rs"
            },
            {
              name: "upload.rs",
              type: "file",
              extension: "rs"
            }
          ]
        },
        {
          name: "models",
          type: "folder",
          children: [
            {
              name: "user.rs",
              type: "file",
              extension: "rs"
            },
            {
              name: "file.rs",
              type: "file",
              extension: "rs"
            }
          ]
        }
      ]
    },
    {
      name: "Cargo.toml",
      type: "file",
      extension: "toml"
    },
    {
      name: "README.md",
      type: "file",
      extension: "md"
    },
    {
      name: "tests",
      type: "folder",
      children: [
        {
          name: "integration_test.rs",
          type: "file",
          extension: "rs"
        }
      ]
    }
  ]
};

interface TreeNodeProps {
  node: FileNode;
  level: number;
  onSelect: (node: FileNode) => void;
  selectedPath: string;
  path: string;
}

const TreeNode: React.FC<TreeNodeProps> = ({ node, level, onSelect, selectedPath, path }) => {
  const [isExpanded, setIsExpanded] = useState(level === 0);
  const currentPath = `${path}/${node.name}`;
  const isSelected = currentPath === selectedPath;
  
  const getFileIcon = (fileName: string, extension?: string) => {
    if (node.type === "folder") return <FolderIcon className="h-4 w-4 text-zipmind-300" />;
    
    switch (extension) {
      case "rs":
        return <CodeIcon className="h-4 w-4 text-orange-400" />;
      case "md":
        return <FileTextIcon className="h-4 w-4 text-blue-400" />;
      case "toml":
      case "json":
        return <DatabaseIcon className="h-4 w-4 text-purple-400" />;
      case "png":
      case "jpg":
      case "svg":
        return <ImageIcon className="h-4 w-4 text-green-400" />;
      default:
        return <FileIcon className="h-4 w-4 text-gray-400" />;
    }
  };
  
  const toggleExpand = (e: React.MouseEvent) => {
    e.stopPropagation();
    setIsExpanded(!isExpanded);
  };

  const handleSelect = () => {
    onSelect(node);
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
        {getFileIcon(node.name, node.extension)}
        <span className="ml-2 text-sm truncate">{node.name}</span>
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
