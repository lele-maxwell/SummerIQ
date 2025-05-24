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
  fileName: string;
  status: 'processing' | 'completed' | 'failed';
  uploadedAt: string;
}

export interface ApiError {
  error: string;
  code: string;
}

export async function uploadProject(
  file: File, 
  token: string
): Promise<UploadResponse> {
  // ... implementation
}
