import { uploadProject } from './upload';
import { UploadResponse, ApiError, API } from '../types/api';

export class ApiClient {
  constructor(private token: string) {}

  async upload(file: File): Promise<UploadResponse> {
    return uploadProject(file, this.token);
  }

  async getUploads(): Promise<UploadResponse[]> {
    const response = await fetch(API.getUploads, {
      headers: {
        'Authorization': `Bearer ${this.token}`
      }
    });
    if (!response.ok) {
      const error: ApiError = await response.json();
      throw new Error(error.error);
    }
    return response.json();
  }

  // Add other API methods
}
