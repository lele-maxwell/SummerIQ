import { useState, useRef, useEffect } from "react";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardFooter, CardHeader } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Avatar } from "@/components/ui/avatar";
import { BrainCogIcon, SendIcon, Loader2Icon, UserIcon } from "lucide-react";
import { sendChatMessage } from "@/api/chat";

interface Message {
  id: number;
  sender: "user" | "ai";
  text: string;
  timestamp: Date;
}

interface ChatInterfaceProps {
  projectName?: string;
  selectedFilePath?: string;
  selectedFileName?: string;
}

const QUICK_SUGGESTIONS = [
  "What is the main purpose of this project?",
  "Explain the project structure",
  "What are the key dependencies?",
  "How does the authentication work?",
  "Show me the API endpoints",
  "What are the best practices used here?"
];

export function ChatInterface({ 
  projectName = "Demo Project",
  selectedFilePath = "",
  selectedFileName = ""
}: ChatInterfaceProps) {
  const [messages, setMessages] = useState<Message[]>([]);
  const [newMessage, setNewMessage] = useState("");
  const [isLoading, setIsLoading] = useState(false);
  const messagesEndRef = useRef<HTMLDivElement>(null);

  // Load messages from localStorage on component mount
  useEffect(() => {
    const savedMessages = localStorage.getItem(`chat_messages_${projectName}`);
    if (savedMessages) {
      try {
        const parsedMessages = JSON.parse(savedMessages);
        // Convert timestamp strings back to Date objects
        const messagesWithDates = parsedMessages.map((msg: any) => ({
          ...msg,
          timestamp: new Date(msg.timestamp)
        }));
        setMessages(messagesWithDates);
      } catch (error) {
        console.error('Error loading chat messages:', error);
        // If there's an error loading, start with the default welcome message
        setMessages([{
          id: 1,
          sender: "ai",
          text: `Hi there! I'm your AI assistant for the "${projectName}" project. I can help you understand the codebase, explain functionality, suggest improvements, and answer any questions you have about the project.

Here are some things you can ask me about:
• Code structure and architecture
• Function and class explanations
• Best practices and improvements
• Dependencies and libraries
• File relationships and imports
• Performance optimizations
• Security considerations

Feel free to ask me anything!`,
          timestamp: new Date(),
        }]);
      }
    } else {
      // No saved messages, start with the default welcome message
      setMessages([{
        id: 1,
        sender: "ai",
        text: `Hi there! I'm your AI assistant for the "${projectName}" project. I can help you understand the codebase, explain functionality, suggest improvements, and answer any questions you have about the project.

Here are some things you can ask me about:
• Code structure and architecture
• Function and class explanations
• Best practices and improvements
• Dependencies and libraries
• File relationships and imports
• Performance optimizations
• Security considerations

Feel free to ask me anything!`,
        timestamp: new Date(),
      }]);
    }
  }, [projectName]);

  // Save messages to localStorage whenever messages change
  useEffect(() => {
    if (messages.length > 0) {
      localStorage.setItem(`chat_messages_${projectName}`, JSON.stringify(messages));
    }
  }, [messages, projectName]);

  const scrollToBottom = () => {
    messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
  };

  useEffect(() => {
    scrollToBottom();
  }, [messages]);

  const handleSend = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!newMessage.trim() || isLoading) return;
    
    const userMessage: Message = {
      id: messages.length + 1,
      sender: "user",
      text: newMessage,
      timestamp: new Date(),
    };
    
    setMessages([...messages, userMessage]);
    const currentMessage = newMessage;
    setNewMessage("");
    setIsLoading(true);
    
    try {
      // Get authentication token
      const token = localStorage.getItem('token');
      
      const response = await sendChatMessage(
        currentMessage, 
        token || undefined,
        projectName,
        selectedFileName,
        selectedFilePath
      );
      
      const aiResponse: Message = {
        id: messages.length + 2,
        sender: "ai",
        text: response.response,
        timestamp: new Date(),
      };
      
      setMessages((prevMessages) => [...prevMessages, aiResponse]);
    } catch (error) {
      console.error('Chat error:', error);
      const errorMessage: Message = {
        id: messages.length + 2,
        sender: "ai",
        text: "I'm sorry, I encountered an error while processing your request. Please try again or check your connection.",
        timestamp: new Date(),
      };
      setMessages((prevMessages) => [...prevMessages, errorMessage]);
    } finally {
      setIsLoading(false);
    }
  };

  const formatTimestamp = (date: Date) => {
    return date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
  };

  const clearChat = () => {
    setMessages([{
      id: 1,
      sender: "ai",
      text: `Hi there! I'm your AI assistant for the "${projectName}" project. I can help you understand the codebase, explain functionality, suggest improvements, and answer any questions you have about the project.

Here are some things you can ask me about:
• Code structure and architecture
• Function and class explanations
• Best practices and improvements
• Dependencies and libraries
• File relationships and imports
• Performance optimizations
• Security considerations

Feel free to ask me anything!`,
      timestamp: new Date(),
    }]);
    localStorage.removeItem(`chat_messages_${projectName}`);
  };

  return (
    <Card className="flex flex-col h-full">
      <CardHeader className="pb-3 pt-3">
        <div className="flex items-center justify-between">
          <div className="flex items-center">
            <BrainCogIcon className="h-5 w-5 text-zipmind-400 mr-2" />
            <h3 className="font-semibold">Project Assistant</h3>
          </div>
          {messages.length > 1 && (
            <Button
              variant="ghost"
              size="sm"
              onClick={clearChat}
              className="text-xs text-muted-foreground hover:text-foreground"
            >
              Clear Chat
            </Button>
          )}
        </div>
        {selectedFileName && (
          <p className="text-xs text-muted-foreground">
            Currently viewing: {selectedFileName}
          </p>
        )}
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
            
            {/* Quick Suggestions */}
            {messages.length === 1 && (
              <div className="flex flex-wrap gap-2 mt-4">
                {QUICK_SUGGESTIONS.map((suggestion, index) => (
                  <Button
                    key={index}
                    variant="outline"
                    size="sm"
                    className="text-xs"
                    onClick={() => {
                      setNewMessage(suggestion);
                      // Auto-send the suggestion after a brief delay
                      setTimeout(() => {
                        const userMessage: Message = {
                          id: messages.length + 1,
                          sender: "user",
                          text: suggestion,
                          timestamp: new Date(),
                        };
                        
                        setMessages([...messages, userMessage]);
                        setNewMessage("");
                        setIsLoading(true);
                        
                        // Send the message
                        sendChatMessage(
                          suggestion, 
                          localStorage.getItem('token') || undefined,
                          projectName,
                          selectedFileName,
                          selectedFilePath
                        ).then(response => {
                          const aiResponse: Message = {
                            id: messages.length + 2,
                            sender: "ai",
                            text: response.response,
                            timestamp: new Date(),
                          };
                          setMessages((prevMessages) => [...prevMessages, aiResponse]);
                        }).catch(error => {
                          console.error('Chat error:', error);
                          const errorMessage: Message = {
                            id: messages.length + 2,
                            sender: "ai",
                            text: "I'm sorry, I encountered an error while processing your request. Please try again or check your connection.",
                            timestamp: new Date(),
                          };
                          setMessages((prevMessages) => [...prevMessages, errorMessage]);
                        }).finally(() => {
                          setIsLoading(false);
                        });
                      }, 100);
                    }}
                  >
                    {suggestion}
                  </Button>
                ))}
              </div>
            )}
            
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
