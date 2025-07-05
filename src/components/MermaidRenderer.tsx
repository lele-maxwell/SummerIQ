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
    <div className="overflow-x-auto bg-white p-4 rounded-lg">
      <Mermaid {...mermaidProps} />
    </div>
  );
};

export default MermaidRenderer;
