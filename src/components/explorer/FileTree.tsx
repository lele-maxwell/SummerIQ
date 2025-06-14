import React from 'react';
import { ChevronRight, ChevronDown, FileIcon, FolderIcon } from 'lucide-react';

interface File {
  name: string;
  path: string;
  type: 'file' | 'directory';
  children?: File[];
}

interface FileTreeProps {
  files: File[];
  onFileSelect: (path: string) => void;
}

export const FileTree: React.FC<FileTreeProps> = ({ files, onFileSelect }) => {
  const [expandedDirs, setExpandedDirs] = React.useState<Set<string>>(new Set());

  const toggleDirectory = (path: string) => {
    const newExpanded = new Set(expandedDirs);
    if (newExpanded.has(path)) {
      newExpanded.delete(path);
    } else {
      newExpanded.add(path);
    }
    setExpandedDirs(newExpanded);
  };

  const renderFile = (file: File, level: number = 0) => {
    const isExpanded = expandedDirs.has(file.path);
    const paddingLeft = `${level * 1.5}rem`;

    if (file.type === 'directory') {
      return (
        <div key={file.path}>
          <div
            className="flex items-center py-1 px-2 hover:bg-accent cursor-pointer"
            style={{ paddingLeft }}
            onClick={() => toggleDirectory(file.path)}
          >
            {isExpanded ? (
              <ChevronDown className="h-4 w-4 mr-1" />
            ) : (
              <ChevronRight className="h-4 w-4 mr-1" />
            )}
            <FolderIcon className="h-4 w-4 mr-2" />
            <span>{file.name}</span>
          </div>
          {isExpanded && file.children && (
            <div>
              {file.children.map((child) => renderFile(child, level + 1))}
            </div>
          )}
        </div>
      );
    }

    return (
      <div
        key={file.path}
        className="flex items-center py-1 px-2 hover:bg-accent cursor-pointer"
        style={{ paddingLeft }}
        onClick={() => onFileSelect(file.path)}
      >
        <FileIcon className="h-4 w-4 mr-2" />
        <span>{file.name}</span>
      </div>
    );
  };

  return (
    <div className="text-sm">
      {files.map((file) => renderFile(file))}
    </div>
  );
};

export default FileTree; 