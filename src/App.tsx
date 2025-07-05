import { useState, useEffect } from "react";
import { TooltipProvider } from "@/components/ui/tooltip";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { BrowserRouter, Routes, Route, Navigate, useNavigate } from "react-router-dom";
import Index from "./pages/Index";
import Dashboard from "./pages/Dashboard";
import About from "./pages/About";
import NotFound from "./pages/NotFound";
import Architecture from "./pages/Architecture";

const queryClient = new QueryClient();

const App = () => {
  const [isAuthenticated, setIsAuthenticated] = useState(() => {
    const token = localStorage.getItem('token');
    return token !== null && token !== '';
  });

  const clearChatSessions = () => {
    Object.keys(localStorage).forEach(key => {
      if (key.startsWith('chat_messages_')) {
        localStorage.removeItem(key);
      }
    });
  };

  const handleLogin = () => {
    setIsAuthenticated(true);
    // Clear all project data and cache on login
    localStorage.removeItem('uploadedFileName');
    localStorage.removeItem('fileStructure');
    Object.keys(localStorage).forEach(key => {
      if (key.startsWith('projectDocCache_')) {
        localStorage.removeItem(key);
      }
    });
    // Clear all chat sessions for a clean slate
    clearChatSessions();
  };

  const handleLogout = () => {
    setIsAuthenticated(false);
    localStorage.removeItem('token');
    // Clear all project documentation cache
    Object.keys(localStorage).forEach(key => {
      if (key.startsWith('projectDocCache_')) {
        localStorage.removeItem(key);
      }
    });
    // Clear all chat sessions
    clearChatSessions();
  };

  return (
    <QueryClientProvider client={queryClient}>
      <TooltipProvider>
        <BrowserRouter>
          <Routes>
            <Route 
              path="/" 
              element={
                <Index 
                  isAuthenticated={isAuthenticated}
                  onLogin={handleLogin}
                  onLogout={handleLogout}
                />
              } 
            />
            <Route 
              path="/dashboard" 
              element={
                isAuthenticated ? (
                  <Dashboard 
                    isAuthenticated={isAuthenticated}
                    onLogout={handleLogout}
                  />
                ) : (
                  <Navigate to="/" replace />
                )
              } 
            />
            <Route 
              path="/about" 
              element={
                <About 
                  isAuthenticated={isAuthenticated}
                  onLogin={handleLogin}
                  onLogout={handleLogout}
                />
              } 
            />
            <Route 
              path="/architecture" 
              element={<Architecture />} 
            />
            <Route path="*" element={<NotFound />} />
          </Routes>
        </BrowserRouter>
      </TooltipProvider>
    </QueryClientProvider>
  );
};

export default App;
