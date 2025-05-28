import { API } from "@/types/api";

// Helper function to get full URL
export const getApiUrl = (endpoint: string): string => {
  return `${API.baseUrl}${endpoint}`;
}; 