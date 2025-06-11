# Getting Started with ZipMind

This guide will help you set up and run the ZipMind project locally. Follow these steps to get your development environment ready.

## Prerequisites

Before you begin, ensure you have the following installed:

- **Node.js** (v18 or later)
- **Rust** (latest stable version)
- **PostgreSQL** (v14 or later)
- **Git**

## Installation

### 1. Clone the Repository

```bash
git clone https://github.com/yourusername/zipmind.git
cd zipmind
```

### 2. Backend Setup

1. **Install Rust Dependencies**
   ```bash
   cd backend
   cargo build
   ```

2. **Configure Environment Variables**
   Create a `.env` file in the backend directory:
   ```env
   DATABASE_URL=postgres://user:password@localhost:5432/zipmind
   JWT_SECRET=your-secret-key
   UPLOAD_DIR=/path/to/uploads
   PORT=8080
   ```

3. **Set Up Database**
   ```bash
   # Create database
   createdb zipmind

   # Run migrations
   cargo run --bin migrate
   ```

4. **Start Backend Server**
   ```bash
   cargo run
   ```

### 3. Frontend Setup

1. **Install Dependencies**
   ```bash
   cd frontend
   npm install
   # or
   yarn install
   ```

2. **Configure Environment Variables**
   Create a `.env` file in the frontend directory:
   ```env
   VITE_API_URL=http://localhost:8080/api
   ```

3. **Start Development Server**
   ```bash
   npm run dev
   # or
   yarn dev
   ```

## Project Structure

```
zipmind/
├── backend/           # Rust backend
│   ├── src/          # Source code
│   ├── migrations/   # Database migrations
│   └── tests/        # Backend tests
├── frontend/         # React frontend
│   ├── src/          # Source code
│   ├── public/       # Static files
│   └── tests/        # Frontend tests
└── docs/            # Documentation
```

## Development Workflow

1. **Start Development Servers**
   - Backend: `cargo run` (in backend directory)
   - Frontend: `npm run dev` (in frontend directory)

2. **Access the Application**
   - Frontend: http://localhost:3000
   - Backend API: http://localhost:8080/api

3. **Testing**
   - Backend tests: `cargo test`
   - Frontend tests: `npm test`

## Common Tasks

### Creating a New User

1. Visit http://localhost:3000/register
2. Fill in the registration form
3. Verify your email (if enabled)

### Uploading a File

1. Log in to the application
2. Click "Upload" in the navigation
3. Drag and drop a ZIP file or click to select
4. Wait for the upload to complete

### Using the AI Chat

1. Select an uploaded file
2. Click "Chat" in the navigation
3. Type your question about the code
4. View the AI's response

## Troubleshooting

### Backend Issues

1. **Database Connection**
   ```bash
   # Check PostgreSQL status
   sudo service postgresql status

   # Verify connection
   psql -d zipmind
   ```

2. **Port Conflicts**
   ```bash
   # Check if port 8080 is in use
   sudo lsof -i :8080
   ```

3. **File Permissions**
   ```bash
   # Set upload directory permissions
   chmod 755 /path/to/uploads
   ```

### Frontend Issues

1. **Node Modules**
   ```bash
   # Clear node_modules and reinstall
   rm -rf node_modules
   npm install
   ```

2. **Build Errors**
   ```bash
   # Clear build cache
   npm run clean
   npm run build
   ```

3. **Development Server**
   ```bash
   # Check if port 3000 is in use
   sudo lsof -i :3000
   ```

## Next Steps

1. **Explore Documentation**
   - [API Documentation](./api/README.md)
   - [Frontend Guide](./frontend/README.md)
   - [Backend Guide](./backend/README.md)

2. **Set Up Development Tools**
   - Install recommended VS Code extensions
   - Configure ESLint and Prettier
   - Set up Git hooks

3. **Start Contributing**
   - Review [Contributing Guide](./contributing.md)
   - Set up development environment
   - Create your first pull request

## Support

If you encounter any issues:

1. Check the [Troubleshooting](#troubleshooting) section
2. Search existing [GitHub Issues](https://github.com/yourusername/zipmind/issues)
3. Create a new issue if needed

## Additional Resources

- [Rust Documentation](https://doc.rust-lang.org/book/)
- [React Documentation](https://reactjs.org/docs/getting-started.html)
- [PostgreSQL Documentation](https://www.postgresql.org/docs/)
- [TypeScript Documentation](https://www.typescriptlang.org/docs/) 