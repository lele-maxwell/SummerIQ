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
  filename: string;
  extracted_files: Array<{
    path: string;
    content?: string;
  }>;
  upload?: {
    extracted_files: Array<{
      path: string;
      content?: string;
    }>;
  };
}

export interface ApiError {
  error: string;
}
