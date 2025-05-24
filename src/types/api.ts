export interface API {
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
  id: string;
  filename: string;
  url: string;
  uploadedAt: string;
}

export interface ApiError {
  error: string;
}
