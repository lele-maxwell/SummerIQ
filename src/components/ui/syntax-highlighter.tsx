import React from 'react';
import { Prism as SyntaxHighlighter } from 'react-syntax-highlighter';
import type { SyntaxHighlighterProps } from 'react-syntax-highlighter';

const CodeBlock = React.forwardRef<
  HTMLPreElement,
  SyntaxHighlighterProps
>(({ children, ...props }, ref) => {
  return (
    <SyntaxHighlighter
      ref={ref}
      {...props}
      customStyle={{
        margin: 0,
        borderRadius: '0.5rem',
        padding: '1rem',
      }}
    >
      {children}
    </SyntaxHighlighter>
  );
});

CodeBlock.displayName = 'CodeBlock';

export { CodeBlock as SyntaxHighlighter };
export type { SyntaxHighlighterProps }; 