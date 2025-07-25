import React, { useState, useEffect } from 'react';
import { API } from '@/types/api';
import { Prism as SyntaxHighlighter } from 'react-syntax-highlighter';
import { vscDarkPlus } from 'react-syntax-highlighter/dist/esm/styles/prism';

function getLanguageFromExtension(path: string): string {
  const ext = path.split('.').pop()?.toLowerCase();
  switch (ext) {
    case 'js': return 'javascript';
    case 'ts': return 'typescript';
    case 'tsx': return 'tsx';
    case 'jsx': return 'jsx';
    case 'json': return 'json';
    case 'py': return 'python';
    case 'rs': return 'rust';
    case 'java': return 'java';
    case 'c': return 'c';
    case 'cpp': return 'cpp';
    case 'cs': return 'csharp';
    case 'go': return 'go';
    case 'html': return 'html';
    case 'css': return 'css';
    case 'scss': return 'scss';
    case 'md': return 'markdown';
    case 'sh': return 'bash';
    case 'yml':
    case 'yaml': return 'yaml';
    case 'xml': return 'xml';
    case 'php': return 'php';
    case 'swift': return 'swift';
    case 'kt': return 'kotlin';
    case 'sql': return 'sql';
    case 'txt': return 'text';
    default: return 'text';
  }
}

const FileViewer: React.FC<{ path: string }> = ({ path }) => {
  const [fileContent, setFileContent] = useState<string>('');
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [analysis, setAnalysis] = useState<{ analysis: string; dependencies: string } | null>(null);
  const [showDependencies, setShowDependencies] = useState(false);

  useEffect(() => {
    const fetchFileContent = async () => {
      setIsLoading(true);
      setError(null);
      try {
        const response = await fetch(`${API.baseUrl}/api/files/content/${path}`);
        if (!response.ok) {
          throw new Error('Failed to fetch file content');
        }
        const data = await response.json();
        setFileContent(data.content);
        
        // Fetch AI analysis
        const analysisResponse = await fetch(`${API.baseUrl}/api/analysis/${path}`);
        if (analysisResponse.ok) {
          const analysisData = await analysisResponse.json();
          setAnalysis(analysisData);
        }
      } catch (err) {
        setError(err instanceof Error ? err.message : 'An error occurred');
      } finally {
        setIsLoading(false);
      }
    };

    fetchFileContent();
  }, [path]);

  if (isLoading) {
    return <div className="p-4">Loading...</div>;
  }

  if (error) {
    return <div className="p-4 text-red-500">Error: {error}</div>;
  }

  return (
    <div className="flex flex-col h-full">
      <div className="flex-1 overflow-auto p-4">
        <SyntaxHighlighter
          language={getLanguageFromExtension(path)}
          style={vscDarkPlus}
          showLineNumbers
          wrapLines
        >
          {fileContent}
        </SyntaxHighlighter>
      </div>
      {analysis && (
        <div className="border-t border-gray-200 p-4 bg-gray-50">
          <div className="flex justify-between items-center mb-2">
            <h3 className="text-lg font-semibold">AI Analysis</h3>
            <button
              onClick={() => setShowDependencies(!showDependencies)}
              className="px-3 py-1 text-sm bg-blue-500 text-white rounded hover:bg-blue-600 transition-colors"
            >
              {showDependencies ? 'Hide Dependencies' : 'Show Dependencies'}
            </button>
          </div>
          <div className="prose max-w-none">
            {showDependencies ? (
              <div>
                <h4 className="font-semibold mb-2">Dependencies:</h4>
                <p className="whitespace-pre-wrap">{analysis.dependencies}</p>
              </div>
            ) : (
              <p className="whitespace-pre-wrap">{analysis.analysis}</p>
            )}
          </div>
        </div>
      )}
    </div>
  );
};

export default FileViewer; 