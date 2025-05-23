
import React from "react";
import { Link } from "react-router-dom";
import { Button } from "@/components/ui/button";
import { GitBranchIcon, BrainCogIcon, LogInIcon } from "lucide-react";

interface HeaderProps {
  isAuthenticated?: boolean;
  onLogin?: () => void;
  onLogout?: () => void;
}

export const Header: React.FC<HeaderProps> = ({ 
  isAuthenticated = false,
  onLogin,
  onLogout
}) => {
  return (
    <header className="border-b border-border bg-background/95 backdrop-blur sticky top-0 z-40">
      <div className="container flex h-16 items-center justify-between">
        <div className="flex items-center gap-2">
          <Link to="/" className="flex items-center gap-2 transition-opacity hover:opacity-80">
            <BrainCogIcon className="h-8 w-8 text-zipmind-400" />
            <span className="text-xl font-semibold">ZipMind</span>
          </Link>
        </div>
        
        <nav className="hidden md:flex items-center gap-6">
          <Link to="/" className="text-sm font-medium hover:text-primary transition-colors">
            Home
          </Link>
          {isAuthenticated && (
            <>
              <Link to="/dashboard" className="text-sm font-medium hover:text-primary transition-colors">
                Dashboard
              </Link>
              <Link to="/projects" className="text-sm font-medium hover:text-primary transition-colors">
                Projects
              </Link>
            </>
          )}
          <Link to="/about" className="text-sm font-medium hover:text-primary transition-colors">
            About
          </Link>
        </nav>

        <div className="flex items-center gap-4">
          {isAuthenticated ? (
            <Button variant="outline" onClick={onLogout}>
              Log out
            </Button>
          ) : (
            <Button onClick={onLogin} className="flex items-center gap-2">
              <LogInIcon className="h-4 w-4" />
              <span>Sign In</span>
            </Button>
          )}
          <a 
            href="https://github.com/yourusername/zipmind" 
            target="_blank" 
            rel="noopener noreferrer" 
            className="inline-flex h-9 w-9 items-center justify-center rounded-md border border-border bg-background text-sm font-medium transition-colors hover:bg-accent"
          >
            <GitBranchIcon className="h-5 w-5" />
            <span className="sr-only">GitHub</span>
          </a>
        </div>
      </div>
    </header>
  );
};
