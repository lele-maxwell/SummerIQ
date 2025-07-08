import React, { useState, useEffect } from 'react';

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
        const response = await fetch(`https://summeriq-production.up.railway.app/api/files/content/${path}`);
        if (!response.ok) {
          throw new Error('Failed to fetch file content');
        }
        const data = await response.json();
        setFileContent(data.content);
        
        // Fetch AI analysis
        const analysisResponse = await fetch(`https://summeriq-production.up.railway.app/api/analysis/${path}`);
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
        <pre className="whitespace-pre-wrap font-mono text-sm">{fileContent}</pre>
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