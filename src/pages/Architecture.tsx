import React from "react";
import { DownloadIcon } from "lucide-react";

const Architecture = () => {
  const handleDownload = () => {
    // Placeholder: Implement download functionality (PDF/Markdown) here
    alert("Download functionality coming soon!");
  };

  return (
    <div className="container mx-auto py-12">
      <div className="flex justify-between items-center mb-8">
        <h1 className="text-4xl font-bold text-center w-full">Project Architecture</h1>
        <button
          className="inline-flex items-center gap-2 px-5 py-2 rounded-lg bg-gradient-to-r from-zipmind-400 to-zipmind-600 text-white font-semibold shadow-lg hover:scale-105 transition-transform duration-200 ml-4"
          onClick={handleDownload}
        >
          <DownloadIcon className="w-5 h-5" />
          Download 
        </button> 
      </div>

      {/* 1. Introduction */}
      <section className="mb-12">
        <h2 className="text-2xl font-semibold mb-2">Introduction</h2>
        <p className="text-muted-foreground mb-4">
          <strong>Welcome to ZipMind!</strong> This project is designed to help developers—especially those new to open source—understand, navigate, and contribute to real-world codebases. Our mission is to make learning project architecture and modern technologies accessible, practical, and engaging.
        </p>
        <ul className="list-disc pl-6 text-muted-foreground mb-4">
          <li><strong>What is ZipMind?</strong> An AI-powered platform for exploring, analyzing, and documenting codebases.</li>
          <li><strong>Who is this for?</strong> Junior developers, students, and anyone looking to level up their understanding of real-world projects.</li>
          <li><strong>What will you learn?</strong> How files and folders are connected, how to read and navigate code, how to use modern tools, and how to make your first contribution.</li>
        </ul>
      </section>

      {/* 1a. How to Learn & Contribute */}
      <section className="mb-12">
        <h2 className="text-2xl font-semibold mb-2">How to Learn & Contribute</h2>
        <p className="text-muted-foreground mb-4">
          <strong>Start here if you're new!</strong> This section will guide you step-by-step through learning the project and what you will be able to do by the end.
        </p>
        <ol className="list-decimal pl-6 text-muted-foreground mb-4 space-y-2">
          <li>
            <strong>Get the Big Picture:</strong> Begin with the architecture diagram below. Understand the main folders and how data flows through the project.
          </li>
          <li>
            <strong>Explore the Folder & File Overview:</strong> Read the purpose of each folder and key file. Use the AI explanations to see how everything connects.
          </li>
          <li>
            <strong>Learn the Technologies:</strong> Review the Technologies Used section. If you're unfamiliar with any, check the provided resources and try small experiments.
          </li>
          <li>
            <strong>Read the Code, Don't Just Skim:</strong> Open files in the explorer, read comments, and use the AI analysis to understand what each part does.
          </li>
          <li>
            <strong>Ask Questions:</strong> If you're stuck, use the chat or community resources. There are no bad questions!
          </li>
        </ol>
        <div className="bg-secondary rounded-lg p-4 mb-4">
          <h3 className="text-lg font-bold mb-2">By the End of This Project, You Will Be Able To:</h3>
          <ul className="list-disc pl-6 text-muted-foreground">
            <li>Understand how a real-world project is structured and how files/folders are connected.</li>
            <li>Navigate and read code confidently, even in large codebases.</li>
            <li>Identify the purpose of each major folder and file.</li>
            <li>Recognize and use modern technologies and tools (React, TypeScript, AI analysis, etc.).</li>
            <li>Follow best practices for code organization and documentation.</li>
            <li>Communicate effectively with other developers and ask good questions.</li>
            <li>Prepare yourself to make meaningful contributions to open source projects.</li>
          </ul>
        </div>
        <div className="bg-card rounded-lg p-4 shadow">
          <h3 className="text-lg font-bold">Tips for Success</h3>
          <ul className="list-disc pl-6 text-muted-foreground">
            <li>Be curious and experiment—try changing things and see what happens.</li>
            <li>Don't be afraid to break things (in your own branch!).</li>
            <li>Read error messages carefully—they often tell you exactly what's wrong.</li>
            <li>Celebrate small wins and keep learning!</li>
          </ul>
        </div>
      </section>

      {/* 2. High-Level Architecture Diagram */}
      <section className="mb-16">
        <h2 className="text-3xl font-bold mb-4 text-center">High-Level Architecture Diagrams</h2>
        <p className="text-lg text-muted-foreground text-center mb-8">
          Visualizing the project is the fastest way to understand how everything fits together. Below are clear, large diagrams showing the main parts of the project and how they interact. Use these as your map while exploring the codebase!
        </p>
        <div className="flex flex-col gap-12 items-center">
          {/* Main Project Structure Diagram */}
          <div className="w-full max-w-5xl h-[420px] bg-secondary rounded-2xl flex flex-col items-center justify-center text-muted-foreground shadow-lg border-2 border-zipmind-400">
            <span className="text-xl font-semibold mb-2">Project Structure Overview</span>
            <span className="mb-4">[AI-Generated Project Structure Diagram Here]</span>
            <span className="text-sm">This diagram shows the main folders, files, and their relationships.</span>
          </div>
          {/* Frontend/Backend/Data Flow Diagram */}
          <div className="w-full max-w-5xl h-[420px] bg-secondary rounded-2xl flex flex-col items-center justify-center text-muted-foreground shadow-lg border-2 border-zipmind-400">
            <span className="text-xl font-semibold mb-2">Frontend, Backend & Data Flow</span>
            <span className="mb-4">[AI-Generated Frontend/Backend/Data Flow Diagram Here]</span>
            <span className="text-sm">This diagram shows how data moves between the frontend, backend, and other services.</span>
          </div>
          {/* Legend/Key for Diagrams */}
          <div className="w-full max-w-2xl bg-card rounded-lg p-4 shadow border border-zipmind-400 mt-4">
            <span className="font-semibold">Legend:</span>
            <ul className="list-disc pl-6 text-muted-foreground text-sm">
              <li><span className="font-bold">Blue boxes</span>: Folders</li>
              <li><span className="font-bold">Green boxes</span>: Files</li>
              <li><span className="font-bold">Arrows</span>: Data or function flow</li>
              <li><span className="font-bold">Dashed lines</span>: Optional or indirect relationships</li>
            </ul>
          </div>
        </div>
        <p className="text-muted-foreground text-center mt-8">
          <em>Note: Diagrams will be AI-generated and downloadable in future updates for your reference.</em>
        </p>
      </section>

      {/* 3. Folder & File Overview */}
      <section className="mb-12">
        <h2 className="text-2xl font-semibold mb-2">Folder & File Overview</h2>
        <p className="text-muted-foreground mb-4">
          This section explains the purpose of each major folder and file in a typical modern React/TypeScript project. Understanding this structure will help you navigate, learn, and contribute with confidence.
        </p>
        <div className="space-y-6">
          {/* Top-level folders */}
          <div className="bg-card rounded-lg p-4 shadow">
            <h3 className="text-lg font-bold">/src</h3>
            <ul className="list-disc pl-6 text-muted-foreground mb-2">
              <li><strong>Main source code folder.</strong> Contains all application logic, components, pages, and utilities.</li>
              <li>Most of your work and learning will happen here.</li>
            </ul>
            <div className="pl-4">
              <h4 className="font-semibold">Key subfolders:</h4>
              <ul className="list-disc pl-6 text-muted-foreground">
                <li><strong>/components</strong>: Reusable UI components (buttons, forms, layout, etc.).</li>
                <li><strong>/pages</strong>: Top-level pages/routes for the app (e.g., Dashboard, About, Index).</li>
                <li><strong>/utils</strong>: Utility/helper functions used throughout the app.</li>
                <li><strong>/hooks</strong>: Custom React hooks for shared logic.</li>
                <li><strong>/layout</strong>: Layout components (headers, footers, navigation bars).</li>
                <li><strong>/analysis</strong>: AI analysis and code understanding features.</li>
                <li><strong>/chat</strong>: Chat interface and related logic.</li>
                <li><strong>/upload</strong>: File upload components and logic.</li>
                <li><strong>/explorer</strong>: File explorer and tree view components.</li>
                <li><strong>/ui</strong>: Shared UI primitives (buttons, dialogs, tooltips, etc.).</li>
              </ul>
              <h4 className="font-semibold mt-2">Key files:</h4>
              <ul className="list-disc pl-6 text-muted-foreground">
                <li><strong>App.tsx</strong>: The root React component; sets up routing and global providers.</li>
                <li><strong>index.tsx</strong>: Entry point for the React app; renders <code>App</code> into the DOM.</li>
                <li><strong>types/</strong>: TypeScript type definitions for the project.</li>
              </ul>
            </div>
          </div>
          <div className="bg-card rounded-lg p-4 shadow">
            <h3 className="text-lg font-bold">/public</h3>
            <ul className="list-disc pl-6 text-muted-foreground">
              <li>Static assets (images, icons, favicon, etc.) served directly by the web server.</li>
              <li>Contains <strong>index.html</strong>, the HTML template for the app.</li>
            </ul>
          </div>
          <div className="bg-card rounded-lg p-4 shadow">
            <h3 className="text-lg font-bold">/docs</h3>
            <ul className="list-disc pl-6 text-muted-foreground">
              <li>Project documentation, guides, and feature explanations.</li>
              <li>Great place to learn about the project's goals, features, and best practices.</li>
            </ul>
          </div>
          <div className="bg-card rounded-lg p-4 shadow">
            <h3 className="text-lg font-bold">/node_modules</h3>
            <ul className="list-disc pl-6 text-muted-foreground">
              <li>All third-party dependencies installed via npm/yarn/pnpm.</li>
              <li><strong>Do not edit files here!</strong> This folder is managed automatically.</li>
            </ul>
          </div>
          <div className="bg-card rounded-lg p-4 shadow">
            <h3 className="text-lg font-bold">/tests</h3>
            <ul className="list-disc pl-6 text-muted-foreground">
              <li>Automated tests for the project (unit, integration, etc.).</li>
              <li>Helps ensure code quality and catch bugs early.</li>
            </ul>
          </div>
          {/* Configuration and special files */}
          <div className="bg-card rounded-lg p-4 shadow">
            <h3 className="text-lg font-bold">Key Configuration & Special Files</h3>
            <ul className="list-disc pl-6 text-muted-foreground">
              <li><strong>package.json</strong>: Lists project dependencies, scripts, and metadata.</li>
              <li><strong>tsconfig.json</strong>: TypeScript configuration (compiler options, paths, etc.).</li>
              <li><strong>.gitignore</strong>: Specifies files/folders to ignore in version control.</li>
              <li><strong>README.md</strong>: Main project overview and instructions.</li>
              <li><strong>pnpm-lock.yaml / yarn.lock / package-lock.json</strong>: Lock files for dependency management.</li>
            </ul>
          </div>
        </div>
      </section>

      {/* 4. Technologies Used */}
      <section className="mb-12">
        <h2 className="text-2xl font-semibold mb-2">Technologies Used</h2>
        <div className="space-y-6">
          {/* React */}
          <div className="bg-card rounded-lg p-4 shadow">
            <h3 className="text-lg font-bold">React</h3>
            <ul className="list-disc pl-6 text-muted-foreground mb-2">
              <li><strong>What is it?</strong> A popular JavaScript library for building user interfaces with reusable components.</li>
              <li><strong>Why is it used?</strong> Enables fast, interactive, and maintainable UIs. Most modern web apps use React or similar frameworks.</li>
              <li><strong>What should you know?</strong> Learn about components, props, state, and hooks. Practice building small UIs and reading React code.</li>
              <li><a href="https://react.dev/" target="_blank" rel="noopener noreferrer" className="text-blue-600 underline">Official React Docs</a></li>
            </ul>
          </div>
          {/* TypeScript */}
          <div className="bg-card rounded-lg p-4 shadow">
            <h3 className="text-lg font-bold">TypeScript</h3>
            <ul className="list-disc pl-6 text-muted-foreground mb-2">
              <li><strong>What is it?</strong> A superset of JavaScript that adds static typing for safer, more reliable code.</li>
              <li><strong>Why is it used?</strong> Helps catch bugs early, improves code quality, and makes large codebases easier to maintain.</li>
              <li><strong>What should you know?</strong> Understand basic types, interfaces, and how TypeScript helps document code. Don't worry about mastering advanced types right away.</li>
              <li><a href="https://www.typescriptlang.org/docs/" target="_blank" rel="noopener noreferrer" className="text-blue-600 underline">Official TypeScript Docs</a></li>
            </ul>
          </div>
          {/* Framer Motion */}
          <div className="bg-card rounded-lg p-4 shadow">
            <h3 className="text-lg font-bold">Framer Motion</h3>
            <ul className="list-disc pl-6 text-muted-foreground mb-2">
              <li><strong>What is it?</strong> A library for creating smooth, modern animations in React apps.</li>
              <li><strong>Why is it used?</strong> Makes the UI feel more dynamic and professional. Animations help guide users and improve experience.</li>
              <li><strong>What should you know?</strong> Learn how to animate components and transitions. Try adding simple animations to your own projects.</li>
              <li><a href="https://www.framer.com/motion/" target="_blank" rel="noopener noreferrer" className="text-blue-600 underline">Framer Motion Docs</a></li>
            </ul>
          </div>
          {/* Lucide Icons */}
          <div className="bg-card rounded-lg p-4 shadow">
            <h3 className="text-lg font-bold">Lucide Icons</h3>
            <ul className="list-disc pl-6 text-muted-foreground mb-2">
              <li><strong>What is it?</strong> An open-source icon library for React and other frameworks.</li>
              <li><strong>Why is it used?</strong> Provides a consistent, modern set of icons for UI elements.</li>
              <li><strong>What should you know?</strong> Learn how to import and use icons in your components.</li>
              <li><a href="https://lucide.dev/" target="_blank" rel="noopener noreferrer" className="text-blue-600 underline">Lucide Icons Docs</a></li>
            </ul>
          </div>
          {/* AI Analysis Engine */}
          <div className="bg-card rounded-lg p-4 shadow">
            <h3 className="text-lg font-bold">AI Analysis Engine</h3>
            <ul className="list-disc pl-6 text-muted-foreground mb-2">
              <li><strong>What is it?</strong> Custom AI-powered tools for analyzing code, generating documentation, and answering questions about the project.</li>
              <li><strong>Why is it used?</strong> Helps you quickly understand unfamiliar code, see relationships, and learn best practices.</li>
              <li><strong>What should you know?</strong> Use the AI features to get explanations, diagrams, and suggestions as you explore the project.</li>
            </ul>
          </div>
          {/* React Router */}
          <div className="bg-card rounded-lg p-4 shadow">
            <h3 className="text-lg font-bold">React Router</h3>
            <ul className="list-disc pl-6 text-muted-foreground mb-2">
              <li><strong>What is it?</strong> A library for handling navigation and routing in React apps.</li>
              <li><strong>Why is it used?</strong> Lets you create multi-page apps with clean URLs and navigation.</li>
              <li><strong>What should you know?</strong> Learn about <code>&lt;Routes&gt;</code>, <code>&lt;Route&gt;</code>, and <code>useNavigate</code> for navigation.</li>
              <li><a href="https://reactrouter.com/en/main" target="_blank" rel="noopener noreferrer" className="text-blue-600 underline">React Router Docs</a></li>
            </ul>
          </div>
          {/* TanStack Query */}
          <div className="bg-card rounded-lg p-4 shadow">
            <h3 className="text-lg font-bold">TanStack Query (React Query)</h3>
            <ul className="list-disc pl-6 text-muted-foreground mb-2">
              <li><strong>What is it?</strong> A library for fetching, caching, and updating data in React apps.</li>
              <li><strong>Why is it used?</strong> Makes data management easier and more efficient, especially for API calls.</li>
              <li><strong>What should you know?</strong> Learn about queries, mutations, and how to use hooks like <code>useQuery</code>.</li>
              <li><a href="https://tanstack.com/query/latest/docs/framework/react/overview" target="_blank" rel="noopener noreferrer" className="text-blue-600 underline">TanStack Query Docs</a></li>
            </ul>
          </div>
          {/* Tailwind CSS */}
          <div className="bg-card rounded-lg p-4 shadow">
            <h3 className="text-lg font-bold">Tailwind CSS</h3>
            <ul className="list-disc pl-6 text-muted-foreground mb-2">
              <li><strong>What is it?</strong> A utility-first CSS framework for rapidly building custom designs.</li>
              <li><strong>Why is it used?</strong> Makes it easy to create responsive, modern UIs with minimal custom CSS.</li>
              <li><strong>What should you know?</strong> Learn how to use utility classes to style components. Try customizing layouts and colors.</li>
              <li><a href="https://tailwindcss.com/docs" target="_blank" rel="noopener noreferrer" className="text-blue-600 underline">Tailwind CSS Docs</a></li>
            </ul>
          </div>
        </div>
      </section>

      {/* 5. How Everything Connects */}
      <section className="mb-16">
        <h2 className="text-3xl font-bold mb-4 text-center">How Everything Connects</h2>
        <p className="text-lg text-muted-foreground text-center mb-8">
          Understanding the flow of data and logic is key to mastering any project. This section provides large, clear diagrams and step-by-step walkthroughs of how typical actions move through the system. Use these examples to help you trace and understand the codebase!
        </p>
        <div className="flex flex-col gap-8 items-center">
          {/* Data Flow/Request Lifecycle Diagram */}
          <div className="w-full max-w-5xl h-[340px] bg-secondary rounded-2xl flex flex-col items-center justify-center text-muted-foreground shadow-lg border-2 border-zipmind-400">
            <span className="text-xl font-semibold mb-2">Request/Data Flow</span>
            <span className="mb-4">[AI-Generated Data Flow Diagram Here]</span>
            <span className="text-sm">This diagram shows how a user action or request travels through the frontend, backend, and database.</span>
          </div>
        </div>
        <div className="mt-8 bg-card rounded-lg p-6 shadow border border-zipmind-400 max-w-4xl mx-auto space-y-8">
          {/* User Upload Flow */}
          <div>
            <h3 className="text-lg font-bold mb-2">User Upload Flow: What Happens When a User Uploads a Project?</h3>
            <ol className="list-decimal pl-6 text-muted-foreground space-y-2 mb-2">
              <li>The user clicks the upload button and selects a ZIP file.</li>
              <li>The frontend sends the file to the backend via an API call.</li>
              <li>The backend extracts the ZIP, analyzes the files, and builds a file tree.</li>
              <li>The backend returns the file structure and analysis results to the frontend.</li>
              <li>The frontend displays the file explorer, analysis, and AI insights to the user.</li>
              <li>The user can now explore, ask questions, and learn from the project structure and documentation.</li>
            </ol>
            <p className="text-muted-foreground text-sm">Tip: You can trace this flow in <code>FileUpload</code>, backend upload handler, and the file explorer components.</p>
          </div>
          {/* File Analysis & AI Flow */}
          <div>
            <h3 className="text-lg font-bold mb-2">File Analysis & AI Flow: How Does the AI Analyze a File?</h3>
            <ol className="list-decimal pl-6 text-muted-foreground space-y-2 mb-2">
              <li>The user selects a file in the file explorer.</li>
              <li>The frontend sends a request to the backend (or AI engine) for analysis of that file.</li>
              <li>The backend/AI engine reads the file, analyzes its code, and generates a summary, dependencies, and suggestions.</li>
              <li>The analysis results are sent back to the frontend and displayed in the analysis panel.</li>
              <li>The user can read the AI-generated explanation, see dependencies, and get suggestions for learning or contributing.</li>
            </ol>
            <p className="text-muted-foreground text-sm">Tip: Look at the <code>AIAnalysis</code> component and related backend routes for this flow.</p>
          </div>
          {/* Chat/Question Flow */}
          <div>
            <h3 className="text-lg font-bold mb-2">Chat/Question Flow: How Can a User Ask Questions About the Project?</h3>
            <ol className="list-decimal pl-6 text-muted-foreground space-y-2 mb-2">
              <li>The user types a question in the chat interface.</li>
              <li>The frontend sends the question to the backend/AI chat engine.</li>
              <li>The AI engine processes the question, references the codebase, and generates a helpful answer.</li>
              <li>The answer is sent back to the frontend and displayed in the chat window.</li>
              <li>The user can ask follow-up questions or explore related files and documentation.</li>
            </ol>
            <p className="text-muted-foreground text-sm">Tip: Check the <code>ChatInterface</code> component and backend chat/AI routes for this flow.</p>
          </div>
          {/* Navigation & State Flow */}
          <div>
            <h3 className="text-lg font-bold mb-2">Navigation & State Flow: How Does the App Handle Navigation and State?</h3>
            <ul className="list-disc pl-6 text-muted-foreground mb-2">
              <li>Navigation between pages (Dashboard, About, Architecture, etc.) is handled by <strong>React Router</strong>.</li>
              <li>State (e.g., uploaded file, selected file, authentication) is managed using React <strong>useState</strong> and <strong>useEffect</strong> hooks.</li>
              <li>Some data (like uploaded file info) is stored in <strong>localStorage</strong> for persistence across sessions.</li>
              <li>API calls and data fetching are managed with <strong>TanStack Query</strong> for efficiency and caching.</li>
            </ul>
            <p className="text-muted-foreground text-sm">Tip: Explore <code>App.tsx</code>, <code>Dashboard.tsx</code>, and the hooks in <code>/src</code> to see how navigation and state are handled.</p>
          </div>
          {/* Tips for Tracing Flows in Code */}
          <div>
            <h3 className="text-lg font-bold mb-2">Tips for Tracing Flows in Code</h3>
            <ul className="list-disc pl-6 text-muted-foreground">
              <li>Start with the UI: Click a button or perform an action, then search for the component handling it.</li>
              <li>Follow the data: See where props, state, or API calls go next.</li>
              <li>Use your browser's dev tools and React DevTools to inspect components and network requests.</li>
              <li>Read comments and documentation in the code for extra guidance.</li>
              <li>Don't hesitate to ask questions or use the AI chat for help understanding tricky flows!</li>
            </ul>
          </div>
        </div>
        <p className="text-muted-foreground text-center mt-8">
          <em>Tip: Refer to the diagrams above as you follow the flow. Visualizing the process will help you understand and remember it!</em>
        </p>
      </section>

      {/* 6. Best Practices & Tips */}
      <section className="mb-12">
        <h2 className="text-2xl font-semibold mb-2">Best Practices & Tips</h2>
        <ul className="list-disc pl-6 text-muted-foreground mb-4">
          <li>Follow the project's coding standards and naming conventions.</li>
          <li>Keep components and utilities modular and reusable.</li>
          <li>Document your code and use clear comments.</li>
          <li>Ask questions and seek help when needed!</li>
        </ul>
        <p className="text-muted-foreground">[AI: Additional project-specific tips and common pitfalls to avoid.]</p>
      </section>

      {/* 7. FAQ & Further Learning */}
      <section className="mb-12">
        <h2 className="text-2xl font-semibold mb-2">FAQ & Further Learning</h2>
        <ul className="list-disc pl-6 text-muted-foreground mb-4">
          <li>How do I get started with my first contribution?</li>
          <li>Where can I find more resources on React/TypeScript/AI?</li>
          <li>Who can I ask for help?</li>
        </ul>
        <p className="text-muted-foreground">[AI: Answers to common questions and links to further resources.]</p>
      </section>
    </div>
  );
};

export default Architecture; 