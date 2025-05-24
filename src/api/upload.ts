import { API } from '../types/api';

export async function uploadProject(
  file: File, 
  token: string
): Promise<UploadResponse> {
  try {
    const formData = new FormData();
    formData.append('file', file);
    
    const response = await fetch(API.upload, {
      method: 'POST',
      body: formData,
      headers: {
        'Authorization': `Bearer ${token}`
      }
    });
    
    if (!response.ok) {
      const error: ApiError = await response.json();
      throw new Error(error.error);
    }
    
    return response.json();
  } catch (error) {
    console.error('Upload failed:', error);
    throw error;
  }
}
