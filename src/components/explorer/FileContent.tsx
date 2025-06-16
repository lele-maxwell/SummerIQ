import React from 'react';
import { Prism as SyntaxHighlighter } from 'react-syntax-highlighter';
import { vscDarkPlus } from 'react-syntax-highlighter/dist/esm/styles/prism';

interface FileContentProps {
  content: string;
  language?: string;
}

export const FileContent: React.FC<FileContentProps> = ({ content, language = 'text' }) => {
  return (
    <div className="relative">
      <SyntaxHighlighter
        language={language}
        style={vscDarkPlus}
        customStyle={{
          margin: 0,
          borderRadius: '0.5rem',
          background: 'hsl(var(--background))',
        }}
        wrapLines
        wrapLongLines
      >
        {content}
      </SyntaxHighlighter>
    </div>
  );
};

export default FileContent; 