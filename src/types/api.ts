export interface API {
  baseUrl: string;
  // Auth
  register: string;
  login: string;
  
  // Upload
  upload: string;
  getUploads: string;
  
  // Chat
  chat: string;
  
  // User
  getMe: string;
}

export const API: API = {
  baseUrl: 'http://127.0.0.1:8080',
  // Auth
  register: '/api/auth/register',
  login: '/api/auth/login',
  
  // Upload
  upload: '/api/upload',
  getUploads: '/api/uploads',
  
  // Chat
  chat: '/api/chat',  
  
  // User
  getMe: '/api/me'
};

export interface FileNode {
  name: string;
  path: string;
  is_dir: boolean;
  children?: FileNode[];
}

export interface UploadResponse {
  filename: string;
  extracted_files: FileNode[];
  upload?: {
    extracted_files: FileNode[];
  };
}

export interface ApiError {
  error: string;
}
