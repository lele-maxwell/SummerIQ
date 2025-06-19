export type FileNode = {
  name: string;
  path: string;
  is_dir: boolean;
  children?: FileNode[] | null;
};
