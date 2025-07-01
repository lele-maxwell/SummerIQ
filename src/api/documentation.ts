import { ApiClient } from './client';
import { API } from '../types/api';

export interface FileAnalysis {
  path: string;
  name: string;
  description: string;
  dependencies: string[];
  relationships: {
    target_file: string;
    relationship_type: string;
    description: string;
  }[];
}

export interface ProjectDocumentation {
  project_name: string;
  description: string;
  architecture: string;
  file_analyses: FileAnalysis[];
  dependencies: string[];
  setup_instructions: string;
}

export const documentationApi = {
  getFileAnalysis: async (filePath: string): Promise<FileAnalysis> => {
    const token = localStorage.getItem('token') || '';
    const apiClient = new ApiClient(token);
    // You may need to implement getFileAnalysis in ApiClient if not present
    const response = await fetch(`${API.baseUrl}/api/documentation/file/${filePath}`, {
      headers: {
        'Authorization': `Bearer ${token}`
      }
    });
    if (!response.ok) throw new Error('Failed to fetch file analysis');
    return response.json();
  },

  getProjectDocumentation: async (projectPath: string = 'current'): Promise<ProjectDocumentation> => {
    const token = localStorage.getItem('token') || '';
    const response = await fetch(`${API.baseUrl}/api/documentation/project/${projectPath}`, {
      headers: {
        'Authorization': `Bearer ${token}`
      }
    });
    if (!response.ok) throw new Error('Failed to fetch project documentation');
    return response.json();
  },

  downloadDocumentation: async (projectPath: string = 'current'): Promise<Blob> => {
    const token = localStorage.getItem('token') || '';
    const response = await fetch(`${API.baseUrl}/api/documentation/download/${projectPath}`, {
      headers: {
        'Authorization': `Bearer ${token}`
      },
      // @ts-ignore
      responseType: 'blob'
    });
    if (!response.ok) throw new Error('Failed to download documentation');
    return response.blob();
  }
}; 