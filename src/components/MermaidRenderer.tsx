import React from 'react';
import Mermaid from 'react-mermaid2';

interface MermaidRendererProps {
  chart: string;
}

const MermaidRenderer: React.FC<MermaidRendererProps> = ({ chart }) => {
  return (
    <div style={{ overflowX: 'auto', background: '#fff', padding: '1rem', borderRadius: '8px' }}>
      <Mermaid chart={chart} />
    </div>
  );
};

export default MermaidRenderer; 