import { API } from '../types/api';

export interface ChatRequest {
  message: string;
  project_name?: string;
  selected_file_name?: string;
  selected_file_path?: string;
}

export interface ChatResponse {
  response: string;
}

export async function sendChatMessage(
  message: string, 
  token?: string,
  projectName?: string,
  selectedFileName?: string,
  selectedFilePath?: string
): Promise<ChatResponse> {
  const headers: Record<string, string> = {
    'Content-Type': 'application/json',
  };

  if (token) {
    headers['Authorization'] = `Bearer ${token}`;
  }

  const requestBody: ChatRequest = {
    message,
    project_name: projectName,
    selected_file_name: selectedFileName,
    selected_file_path: selectedFilePath,
  };

  const response = await fetch(`${API.baseUrl}${API.chat}`, {
    method: 'POST',
    headers,
    body: JSON.stringify(requestBody),
  });

  if (!response.ok) {
    const error = await response.json();
    throw new Error(error.error || 'Failed to send chat message');
  }

  return response.json();
} 