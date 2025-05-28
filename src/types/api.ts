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
  baseUrl: 'http://localhost:3000',
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
  key: string;
  fileName: string;
}

export interface ApiError {
  error: string;
}
