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
    return response.json();
  }

  // Add other API methods
}
