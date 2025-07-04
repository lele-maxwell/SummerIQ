import React, { useState } from 'react';
import Mermaid from 'react-mermaid2';

interface MermaidRendererProps {
  chart: string;
  name?: string;
  config?: object;
}

const MermaidRenderer: React.FC<MermaidRendererProps> = ({ chart, name, config }) => {
  const [hasError, setHasError] = useState(false);

  if (hasError) {
    return (
      <div className="text-red-500 p-4 bg-red-50 border border-red-200 rounded-lg">
        <p className="text-sm">Unable to render diagram. Please check the Mermaid syntax.</p>
      </div>
    );
  }

  // Only pass name/config if defined
  const mermaidProps: any = { chart };
  if (name) mermaidProps.name = name;
  if (config) mermaidProps.config = config;

  return (
    <div
      className="flex justify-center items-center bg-white p-6 rounded-lg shadow-lg"
      style={{ minHeight: 400, width: '100%', maxWidth: '100%', overflowX: 'auto' }}
    >
      <div style={{ minWidth: 600, width: '100%', maxWidth: 1200 }}>
        <Mermaid {...mermaidProps} />
      </div>
    </div>
  );
};

export default MermaidRenderer;
