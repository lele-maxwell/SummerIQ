
export type FileNode = {
  name: string;
  type: "file" | "folder";
  extension?: string;
  children?: FileNode[];
};
