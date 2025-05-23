
import { useState, useRef, useEffect } from "react";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardFooter, CardHeader } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Avatar } from "@/components/ui/avatar";
import { BrainCogIcon, SendIcon, Loader2Icon, UserIcon } from "lucide-react";

interface Message {
  id: number;
  sender: "user" | "ai";
  text: string;
  timestamp: Date;
}

interface ChatInterfaceProps {
  projectName?: string;
}

export function ChatInterface({ projectName = "Demo Project" }: ChatInterfaceProps) {
  const [messages, setMessages] = useState<Message[]>([
    {
      id: 1,
      sender: "ai",
      text: "Hi there! I've analyzed your project. Feel free to ask me any questions about the code, structure, or functionality.",
      timestamp: new Date(),
    },
  ]);
  const [newMessage, setNewMessage] = useState("");
  const [isLoading, setIsLoading] = useState(false);
  const messagesEndRef = useRef<HTMLDivElement>(null);

  const scrollToBottom = () => {
    messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
  };

  useEffect(() => {
    scrollToBottom();
  }, [messages]);

  const handleSend = (e: React.FormEvent) => {
    e.preventDefault();
    if (!newMessage.trim()) return;
    
    const userMessage: Message = {
      id: messages.length + 1,
      sender: "user",
      text: newMessage,
      timestamp: new Date(),
    };
    
    setMessages([...messages, userMessage]);
    setNewMessage("");
    setIsLoading(true);
    
    // Simulate AI response with sample responses based on keywords
    setTimeout(() => {
      const lowerCaseMessage = newMessage.toLowerCase();
      let aiResponseText = "I'm not sure how to answer that question. Could you provide more details about what you're looking for?";
      
      if (lowerCaseMessage.includes("main") || lowerCaseMessage.includes("purpose")) {
        aiResponseText = `This project appears to be a file analysis system. The main entry point is in src/main.rs, which initializes the web server and database connection. The core functionality is implemented in the routes/ and models/ folders.`;
      } else if (lowerCaseMessage.includes("auth") || lowerCaseMessage.includes("login")) {
        aiResponseText = `The authentication system is implemented in src/routes/auth.rs. It uses JWT tokens for session management and bcrypt for password hashing. The login route validates credentials against the database and returns a signed token.`;
      } else if (lowerCaseMessage.includes("file") || lowerCaseMessage.includes("upload")) {
        aiResponseText = `File uploads are handled in src/routes/upload.rs. It accepts zip files, stores them in MinIO, and then processes each file for analysis. The supported file types include Rust, Python, JavaScript, and Markdown.`;
      } else if (lowerCaseMessage.includes("database") || lowerCaseMessage.includes("storage")) {
        aiResponseText = `The project uses PostgreSQL for storing user data, file metadata, and analysis results. There's also MinIO integration for storing the actual zip files. The database schema includes tables for users, sessions, uploads, and files.`;
      } else if (lowerCaseMessage.includes("api") || lowerCaseMessage.includes("endpoint")) {
        aiResponseText = `The API has several endpoints:\n- POST /auth/register: Create new user\n- POST /auth/login: Authenticate user\n- POST /upload: Upload zip file\n- GET /files/:id: Get file details\n- POST /chat: Ask questions about files`;
      }
      
      const aiResponse: Message = {
        id: messages.length + 2,
        sender: "ai",
        text: aiResponseText,
        timestamp: new Date(),
      };
      
      setMessages((prevMessages) => [...prevMessages, aiResponse]);
      setIsLoading(false);
    }, 1500);
  };

  const formatTimestamp = (date: Date) => {
    return date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
  };

  return (
    <Card className="flex flex-col h-full">
      <CardHeader className="pb-3 pt-3">
        <div className="flex items-center">
          <BrainCogIcon className="h-5 w-5 text-zipmind-400 mr-2" />
          <h3 className="font-semibold">Project Assistant</h3>
        </div>
      </CardHeader>
      <CardContent className="flex-grow overflow-hidden p-0">
        <ScrollArea className="h-full p-4">
          <div className="space-y-4">
            {messages.map((message) => (
              <div
                key={message.id}
                className={`flex ${
                  message.sender === "user" ? "justify-end" : "justify-start"
                }`}
              >
                <div className="flex gap-3 max-w-[80%]">
                  {message.sender === "ai" && (
                    <Avatar className="bg-primary h-8 w-8">
                      <BrainCogIcon className="h-4 w-4" />
                    </Avatar>
                  )}
                  <div>
                    <div
                      className={`rounded-lg px-3 py-2 text-sm ${
                        message.sender === "user"
                          ? "bg-primary text-primary-foreground"
                          : "bg-muted text-foreground"
                      }`}
                    >
                      <p style={{ whiteSpace: 'pre-line' }}>{message.text}</p>
                    </div>
                    <p className="text-xs text-muted-foreground mt-1">
                      {formatTimestamp(message.timestamp)}
                    </p>
                  </div>
                  {message.sender === "user" && (
                    <Avatar className="bg-secondary h-8 w-8">
                      <UserIcon className="h-4 w-4" />
                    </Avatar>
                  )}
                </div>
              </div>
            ))}
            {isLoading && (
              <div className="flex justify-start">
                <div className="flex gap-3 max-w-[80%]">
                  <Avatar className="bg-primary h-8 w-8">
                    <BrainCogIcon className="h-4 w-4" />
                  </Avatar>
                  <div>
                    <div className="rounded-lg px-3 py-2 text-sm bg-muted">
                      <Loader2Icon className="h-4 w-4 animate-spin" />
                    </div>
                  </div>
                </div>
              </div>
            )}
            <div ref={messagesEndRef} />
          </div>
        </ScrollArea>
      </CardContent>
      <CardFooter className="pt-3 pb-3">
        <form onSubmit={handleSend} className="flex w-full gap-2">
          <Input
            placeholder={`Ask something about ${projectName}...`}
            value={newMessage}
            onChange={(e) => setNewMessage(e.target.value)}
            disabled={isLoading}
            className="flex-grow"
          />
          <Button type="submit" size="icon" disabled={!newMessage.trim() || isLoading}>
            <SendIcon className="h-4 w-4" />
          </Button>
        </form>
      </CardFooter>
    </Card>
  );
}
