declare module 'react-mermaid2' {
  import * as React from 'react';
  interface MermaidProps {
    chart: string;
    key?: string;
    onError?: () => void;
  }
  const Mermaid: React.FC<MermaidProps>;
  export default Mermaid;
} 