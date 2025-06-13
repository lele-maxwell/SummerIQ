export interface API {
  baseUrl: string;
  // Auth
  register: string;
  login: string;
  
  // Upload
  upload: string;
  getUploads: string;
  
  // Chat
  ask: string;
  
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
  ask: '/api/ask',  
  
  // User
  getMe: '/api/me'
};

export interface UploadResponse {
  file_id: string;
  filename: string;
  message: string;
  upload: {
    id: string;
    filename: string;
    mime_type: string;
    created_at: string;
    extracted_files: string[];
    extraction_path?: string;
  };
}

export interface ApiError {
  error: string;
}
