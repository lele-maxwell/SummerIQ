
import { FileNode } from "./types";

// Mock file structure for demo purposes
export const demoFileStructure: FileNode = {
  name: "project-name",
  type: "folder",
  children: [
    {
      name: "src",
      type: "folder",
      children: [
        {
          name: "main.rs",
          type: "file",
          extension: "rs"
        },
        {
          name: "routes",
          type: "folder",
          children: [
            {
              name: "auth.rs",
              type: "file",
              extension: "rs"
            },
            {
              name: "upload.rs",
              type: "file",
              extension: "rs"
            }
          ]
        },
        {
          name: "models",
          type: "folder",
          children: [
            {
              name: "user.rs",
              type: "file",
              extension: "rs"
            },
            {
              name: "file.rs",
              type: "file",
              extension: "rs"
            }
          ]
        }
      ]
    },
    {
      name: "Cargo.toml",
      type: "file",
      extension: "toml"
    },
    {
      name: "README.md",
      type: "file",
      extension: "md"
    },
    {
      name: "tests",
      type: "folder",
      children: [
        {
          name: "integration_test.rs",
          type: "file",
          extension: "rs"
        }
      ]
    }
  ]
};
