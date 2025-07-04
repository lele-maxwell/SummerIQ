import React, { useState } from 'react';
import Mermaid from 'react-mermaid2';

interface MermaidRendererProps {
  chart: string;
}

const MermaidRenderer: React.FC<MermaidRendererProps> = ({ chart }) => {
  const [hasError, setHasError] = useState(false);

  // Simple error boundary for Mermaid rendering
  if (hasError) {
    return <div style={{ color: 'red', padding: '1rem' }}>Diagram failed to render.</div>;
  }

  return (
    <div style={{ overflowX: 'auto', background: '#fff', padding: '1rem', borderRadius: '8px' }}>
      <Mermaid
        chart={chart}
        key={chart}
        onError={() => setHasError(true)}
      />
    </div>
  );
};

export default MermaidRenderer;
