import React from "react";
import { Header } from "@/components/layout/Header";
import { BrainCogIcon, CodeIcon, FileCodeIcon, MessageSquareTextIcon, BookOpenIcon, GraduationCapIcon } from "lucide-react";

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
              Your AI-powered learning companion for understanding project architecture
            </p>
          </div>

          <div className="grid gap-8 md:grid-cols-2 mb-12">
            <FeatureCard
              icon={<BookOpenIcon className="h-8 w-8 text-zipmind-400" />}
              title="Project Understanding"
              description="Learn how different parts of a project connect and work together through AI-generated documentation and visualizations."
            />
            <FeatureCard
              icon={<GraduationCapIcon className="h-8 w-8 text-zipmind-400" />}
              title="Learning Platform"
              description="Discover modern technologies, patterns, and best practices used in real-world projects through interactive learning."
            />
            <FeatureCard
              icon={<FileCodeIcon className="h-8 w-8 text-zipmind-400" />}
              title="Architecture Analysis"
              description="Get detailed insights into project structure, file relationships, and architectural patterns with visual diagrams."
            />
            <FeatureCard
              icon={<MessageSquareTextIcon className="h-8 w-8 text-zipmind-400" />}
              title="Interactive Learning"
              description="Ask questions about code, get explanations, and learn from AI-powered insights about project architecture."
            />
          </div>

          <div className="bg-card border rounded-xl p-8 mb-12">
            <h2 className="text-2xl font-bold mb-4">Our Mission</h2>
            <p className="text-muted-foreground mb-4">
              At ZipMind, we're dedicated to bridging the gap between knowing how to code and understanding how entire projects work. 
              We believe that every developer, especially those early in their career, should be able to understand and contribute to 
              complex projects with confidence.
            </p>
            <p className="text-muted-foreground mb-4">
              Our AI-powered platform helps developers learn project architecture, understand file relationships, and discover modern 
              technologies through interactive documentation, visualizations, and guided learning paths.
            </p>
            <p className="text-muted-foreground">
              Whether you're a junior developer learning to contribute to open-source projects or a senior developer creating 
              documentation for your team, ZipMind provides the tools and insights you need to understand and share project knowledge effectively.
            </p>
          </div>

          <div className="text-center">
            <h2 className="text-2xl font-bold mb-4">Start Learning Today</h2>
            <p className="text-muted-foreground mb-8">
              Join developers who are using ZipMind to understand project architecture and contribute with confidence.
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