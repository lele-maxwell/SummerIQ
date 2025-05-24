import { ApiClient } from '../api/client';

export default {
  setup() {
    const token = useAuthToken(); // Your auth token hook
    const api = new ApiClient(token.value);

    const handleUpload = async (file: File) => {
      try {
        const result = await api.upload(file);
        console.log('Upload successful:', result);
      } catch (error) {
        console.error('Upload failed:', error);
      }
    };

    return {
      handleUpload
    };
  }
};
