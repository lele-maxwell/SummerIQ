# Frontend Documentation

## Overview

The ZipMind frontend is built with React and TypeScript, using modern web development practices and tools. This documentation will guide you through the frontend architecture, components, and development workflow.

## Tech Stack

- **Framework**: React 18
- **Language**: TypeScript
- **Styling**: Tailwind CSS
- **State Management**: React Hooks
- **Routing**: React Router
- **HTTP Client**: Axios
- **Build Tool**: Vite

## Project Structure

```
src/
├── components/         # Reusable UI components
│   ├── layout/        # Layout components (Header, Navigation)
│   ├── chat/          # Chat interface components
│   ├── upload/        # File upload components
│   └── ui/            # Common UI elements
├── pages/             # Page components
├── types/             # TypeScript type definitions
├── services/          # API and service integrations
├── hooks/             # Custom React hooks
├── utils/             # Utility functions
└── styles/            # Global styles and Tailwind config
```

## Key Components

### Layout Components

- **Header**: Main navigation header with authentication state
- **Navigation**: Responsive navigation menu
- **Layout**: Page layout wrapper

### Feature Components

- **FileUpload**: Drag-and-drop file upload interface
- **FileExplorer**: Code file browser and viewer
- **ChatInterface**: AI chat interaction component
- **AIAnalysis**: Code analysis results display

## Getting Started

1. **Installation**
   ```bash
   npm install
   # or
   yarn install
   ```

2. **Development**
   ```bash
   npm run dev
   # or
   yarn dev
   ```

3. **Build**
   ```bash
   npm run build
   # or
   yarn build
   ```

## Component Documentation

### FileUpload Component

The `FileUpload` component handles file uploads with the following features:

- Drag-and-drop support
- File type validation
- Upload progress indication
- Error handling
- Authentication integration

```typescript
interface FileUploadProps {
  onUploadSuccess: (fileInfo: FileInfo) => void;
  onUploadError: (error: Error) => void;
}
```

### ChatInterface Component

The `ChatInterface` component provides AI chat functionality:

- Real-time message updates
- Message history
- Code snippet support
- Markdown rendering
- Error handling

```typescript
interface ChatInterfaceProps {
  fileId: string;
  onError: (error: Error) => void;
}
```

## State Management

The application uses React's built-in state management with hooks:

- `useState` for local component state
- `useContext` for global state
- Custom hooks for reusable logic

## API Integration

API calls are handled through the `api.ts` service:

```typescript
// Example API call
const uploadFile = async (file: File): Promise<FileInfo> => {
  const formData = new FormData();
  formData.append('file', file);
  return await api.post('/upload', formData);
};
```

## Styling

The project uses Tailwind CSS for styling with a custom configuration:

- Responsive design
- Dark mode support
- Custom color scheme
- Component-specific styles

## Testing

Frontend testing is implemented using:

- Jest for unit tests
- React Testing Library for component tests
- Cypress for E2E tests

## Best Practices

1. **Component Structure**
   - Keep components small and focused
   - Use TypeScript interfaces
   - Implement proper error handling
   - Follow React hooks guidelines

2. **Code Style**
   - Use ESLint and Prettier
   - Follow TypeScript best practices
   - Write meaningful comments
   - Maintain consistent naming

3. **Performance**
   - Implement proper memoization
   - Use lazy loading
   - Optimize bundle size
   - Monitor performance metrics

## Contributing

See the [Contributing Guide](../contributing.md) for detailed information about:

- Setting up the development environment
- Code style guidelines
- Pull request process
- Testing requirements

## Troubleshooting

Common issues and solutions:

1. **Build Errors**
   - Clear node_modules and reinstall
   - Check TypeScript configurations
   - Verify dependency versions

2. **Runtime Errors**
   - Check browser console
   - Verify API endpoints
   - Validate authentication state

3. **Styling Issues**
   - Check Tailwind configuration
   - Verify class names
   - Inspect responsive breakpoints 