
import { FolderIcon, FileIcon, FileTextIcon, CodeIcon, DatabaseIcon, ImageIcon } from "lucide-react";

export const getFileIcon = (fileName: string, nodeType: "file" | "folder", extension?: string) => {
  if (nodeType === "folder") return <FolderIcon className="h-4 w-4 text-green-400" />;
  
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
      return <ImageIcon className="h-4 w-4 text-green-300" />;
    default:
      return <FileIcon className="h-4 w-4 text-gray-400" />;
  }
};
