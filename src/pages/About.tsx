import React from "react";
import { Header } from "@/components/layout/Header";
import { BrainCogIcon, CodeIcon, FileCodeIcon, MessageSquareTextIcon } from "lucide-react";

interface AboutProps {
  isAuthenticated: boolean;
  onLogin: () => void;
  onLogout: () => void;
}

const About = ({ isAuthenticated, onLogin, onLogout }: AboutProps) => {
  return (
    <div className="min-h-screen flex flex-col bg-background">
      <Header 
        isAuthenticated={isAuthenticated} 
        onLogin={onLogin}
        onLogout={onLogout}
      />
      
      <main className="flex-grow container mx-auto py-12 px-4">
        <div className="max-w-4xl mx-auto">
          <div className="text-center mb-12">
            <div className="inline-flex items-center justify-center p-3 bg-secondary rounded-full mb-6">
              <BrainCogIcon className="h-12 w-12 text-zipmind-400" />
            </div>
            <h1 className="text-4xl font-bold mb-4">About ZipMind</h1>
            <p className="text-xl text-muted-foreground">
              Your AI-powered code analysis companion
            </p>
          </div>

          <div className="grid gap-8 md:grid-cols-2 mb-12">
            <FeatureCard
              icon={<CodeIcon className="h-8 w-8 text-zipmind-400" />}
              title="Code Analysis"
              description="Upload your codebase and get instant insights into its structure, patterns, and potential improvements."
            />
            <FeatureCard
              icon={<FileCodeIcon className="h-8 w-8 text-zipmind-400" />}
              title="File Explorer"
              description="Navigate through your project files with ease, understanding the relationships between different components."
            />
            <FeatureCard
              icon={<MessageSquareTextIcon className="h-8 w-8 text-zipmind-400" />}
              title="AI Chat"
              description="Ask questions about your code and get intelligent responses powered by advanced AI models."
            />
            <FeatureCard
              icon={<BrainCogIcon className="h-8 w-8 text-zipmind-400" />}
              title="Smart Insights"
              description="Receive detailed analysis and recommendations to improve your code quality and maintainability."
            />
          </div>

          <div className="bg-card border rounded-xl p-8 mb-12">
            <h2 className="text-2xl font-bold mb-4">Our Mission</h2>
            <p className="text-muted-foreground mb-4">
              At ZipMind, we're dedicated to making code analysis and understanding more accessible and efficient. 
              Our AI-powered platform helps developers quickly grasp complex codebases, identify potential issues, 
              and make informed decisions about their code.
            </p>
            <p className="text-muted-foreground">
              Whether you're working on a personal project or a large-scale application, ZipMind provides the tools 
              and insights you need to write better code and become a more effective developer.
            </p>
          </div>

          <div className="text-center">
            <h2 className="text-2xl font-bold mb-4">Get Started Today</h2>
            <p className="text-muted-foreground mb-8">
              Join thousands of developers who are already using ZipMind to improve their coding workflow.
            </p>
            <a 
              href="/" 
              className="inline-flex items-center justify-center px-6 py-3 rounded-lg bg-zipmind-400 text-white font-medium hover:bg-zipmind-500 transition-colors"
            >
              Try ZipMind Now
            </a>
          </div>
        </div>
      </main>
    </div>
  );
};

const FeatureCard = ({ 
  icon, 
  title, 
  description 
}: { 
  icon: React.ReactNode; 
  title: string; 
  description: string;
}) => {
  return (
    <div className="bg-card border rounded-xl p-6">
      <div className="bg-secondary inline-flex rounded-lg p-3 mb-4">{icon}</div>
      <h3 className="text-xl font-semibold mb-2">{title}</h3>
      <p className="text-muted-foreground">{description}</p>
    </div>
  );
};

export default About; 