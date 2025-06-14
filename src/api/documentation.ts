import { apiClient } from './client';

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
    const response = await apiClient.get(`/api/documentation/file/${filePath}`);
    return response.data;
  },

  getProjectDocumentation: async (projectPath: string = 'current'): Promise<ProjectDocumentation> => {
    const response = await apiClient.get(`/api/documentation/project/${projectPath}`);
    return response.data;
  },

  downloadDocumentation: async (projectPath: string = 'current'): Promise<Blob> => {
    const response = await apiClient.get(`/api/documentation/download/${projectPath}`, {
      responseType: 'blob'
    });
    return response.data;
  }
}; 