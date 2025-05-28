import { API, UploadResponse } from '../types/api';

export async function uploadProject(
  file: File, 
  token: string
): Promise<UploadResponse> {
  try {
    console.log('Starting upload with token:', token ? 'Token present' : 'No token');
    const formData = new FormData();
    formData.append('file', file);
    
    const url = `${API.baseUrl}${API.upload}`;
    console.log('Upload URL:', url);
    
    const response = await fetch(url, {
      method: 'POST',
      body: formData,
      headers: {
        'Authorization': `Bearer ${token}`
        // Don't set Content-Type header - browser will set it automatically with boundary
      },
      credentials: 'include' // Include credentials for CORS
    });
    
    console.log('Response status:', response.status);
    console.log('Response headers:', Object.fromEntries(response.headers.entries()));
    
    if (!response.ok) {
      let errorMessage = 'Upload failed';
      try {
        // First try to get the response text to see what we're dealing with
        const responseText = await response.text();
        console.log('Error response text:', responseText);
        
        if (responseText) {
          try {
            const errorData = JSON.parse(responseText);
            console.error('Error response JSON:', errorData);
            errorMessage = errorData.error || errorData.message || errorMessage;
          } catch (parseError) {
            console.error('Failed to parse error response as JSON:', parseError);
            errorMessage = responseText || errorMessage;
          }
        } else {
          errorMessage = response.statusText || errorMessage;
        }
      } catch (e) {
        console.error('Failed to read error response:', e);
        errorMessage = response.statusText || errorMessage;
      }
      throw new Error(errorMessage);
    }
    
    const data = await response.json();
    console.log('Upload successful:', data);
    return data;
  } catch (error) {
    console.error('Upload failed:', error);
    throw error;
  }
}
