import React from "react";
import { Link, useLocation } from "react-router-dom";
import { Button } from "@/components/ui/button";
import { GitBranchIcon, LogInIcon, HomeIcon, InfoIcon, LayoutDashboardIcon, LogOutIcon } from "lucide-react";
import { cn } from "@/lib/utils";

interface HeaderProps {
  isAuthenticated: boolean;
  onLogin: () => void;
  onLogout: () => void;
}

export function Header({ isAuthenticated, onLogin, onLogout }: HeaderProps) {
  const location = useLocation();
  const isActive = (path: string) => location.pathname === path;

  return (
    <header className="border-b border-border bg-background/95 backdrop-blur sticky top-0 z-40">
      <div className="container flex h-24 items-center justify-between">
        <div className="flex items-center gap-2">
          <Link to="/" className="flex items-center gap-2 transition-opacity hover:opacity-80">
            <div className="relative h-20 w-20">
              <img 
                src="ChatGPT Image Jun 18, 2025, 10_12_16 AM.png" 
                alt="ZipMind Logo" 
                className="absolute inset-0 h-full w-full object-contain"
                style={{
                  filter: 'brightness(1.2) contrast(1.2)',
                  mixBlendMode: 'multiply'
                }}
              />
            </div>
            <span className="text-2xl font-semibold">ZipMind</span>
          </Link>
        </div>
        
        <nav className="hidden md:flex items-center gap-4">
          <Link 
            to="/" 
            className={cn(
              "flex items-center gap-2 px-4 py-2 rounded-lg border-2 transition-all duration-200",
              isActive("/")
                ? "border-zipmind-400 bg-zipmind-400/10 text-zipmind-400"
                : "border-border hover:border-zipmind-400/50 hover:bg-zipmind-400/5"
            )}
          >
            <HomeIcon className="h-4 w-4" />
            <span className="font-medium">Home</span>
          </Link>

          {isAuthenticated && (
            <Link 
              to="/dashboard" 
              className={cn(
                "flex items-center gap-2 px-4 py-2 rounded-lg border-2 transition-all duration-200",
                isActive("/dashboard")
                  ? "border-zipmind-400 bg-zipmind-400/10 text-zipmind-400"
                  : "border-border hover:border-zipmind-400/50 hover:bg-zipmind-400/5"
              )}
            >
              <LayoutDashboardIcon className="h-4 w-4" />
              <span className="font-medium">Dashboard</span>
            </Link>
          )}

          <Link 
            to="/about" 
            className={cn(
              "flex items-center gap-2 px-4 py-2 rounded-lg border-2 transition-all duration-200",
              isActive("/about")
                ? "border-zipmind-400 bg-zipmind-400/10 text-zipmind-400"
                : "border-border hover:border-zipmind-400/50 hover:bg-zipmind-400/5"
            )}
          >
            <InfoIcon className="h-4 w-4" />
            <span className="font-medium">About</span>
          </Link>
        </nav>

        <div className="flex items-center gap-4">
          {isAuthenticated ? (
            <Button 
              variant="outline" 
              onClick={onLogout}
              className="flex items-center gap-2 hover:bg-red-50 hover:text-red-500 hover:border-red-200"
            >
              <LogOutIcon className="h-4 w-4" />
              <span>Log out</span>
            </Button>
          ) : (
            <Button 
              onClick={onLogin} 
              className="flex items-center gap-2 bg-zipmind-400 hover:bg-zipmind-500"
            >
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
}
