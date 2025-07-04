# Backend Documentation

## Overview

The ZipMind backend is built with Rust using the Actix-web framework, providing a robust and performant API for file handling, authentication, and AI integration.

## Tech Stack

- **Language**: Rust
- **Framework**: Actix-web
- **Database**: PostgreSQL
- **ORM**: SQLx
- **Authentication**: JWT
- **File Storage**: Local filesystem
- **Testing**: Rust's built-in testing framework

## Project Structure

```
src/
├── config.rs          # Configuration management
├── error.rs          # Error handling and types
├── handlers/         # API endpoint handlers
│   ├── auth.rs      # Authentication endpoints
│   ├── upload.rs    # File upload endpoints
│   └── chat.rs      # Chat endpoints
├── models/          # Database models
├── services/        # Business logic
│   ├── auth.rs     # Authentication service
│   ├── storage.rs  # File storage service
│   └── ai.rs       # AI integration service
└── main.rs         # Application entry point
```

## API Endpoints

### Authentication

```http
POST /api/auth/register
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "securepassword"
}
```

```http
POST /api/auth/login
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "securepassword"
}
```

### File Upload

```http
POST /api/upload
Authorization: Bearer <jwt_token>
Content-Type: multipart/form-data

file: <file_data>
```

### Chat

```http
POST /api/chat
Authorization: Bearer <jwt_token>
Content-Type: application/json

{
  "file_id": "uuid",
  "message": "What does this code do?"
}
```

## Database Schema

### Users Table

```sql
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
```

### File Uploads Table

```sql
CREATE TABLE file_uploads (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id),
    filename VARCHAR(255) NOT NULL,
    original_filename VARCHAR(255) NOT NULL,
    file_size BIGINT NOT NULL,
    file_type VARCHAR(50) NOT NULL,
    storage_path VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
```

## Services

### Authentication Service

The `AuthService` handles user registration, login, and JWT token management:

```rust
pub struct AuthService {
    pool: PgPool,
    jwt_secret: String,
}

impl AuthService {
    pub async fn register(&self, email: &str, password: &str) -> Result<User>;
    pub async fn login(&self, email: &str, password: &str) -> Result<String>;
    pub fn verify_token(&self, token: &str) -> Result<Claims>;
}
```

### Storage Service

The `StorageService` manages file uploads and storage:

```rust
pub struct StorageService {
    upload_dir: PathBuf,
}

impl StorageService {
    pub async fn save_file(&self, file: MultipartFile) -> Result<FileInfo>;
    pub async fn get_file(&self, file_id: Uuid) -> Result<FileInfo>;
    pub async fn delete_file(&self, file_id: Uuid) -> Result<()>;
}
```

## Error Handling

The application uses a custom error type for consistent error handling:

```rust
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Authentication error: {0}")]
    Auth(String),
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("File error: {0}")]
    File(String),
    #[error("Validation error: {0}")]
    Validation(String),
}
```

## Configuration

Configuration is managed through environment variables:

```env
DATABASE_URL=postgres://user:password@localhost:5432/zipmind
JWT_SECRET=your-secret-key
UPLOAD_DIR=/path/to/uploads
PORT=8080
```

## Security

1. **Authentication**
   - JWT-based authentication
   - Password hashing with bcrypt
   - Token expiration and refresh

2. **File Security**
   - Secure file storage
   - File type validation
   - Size limits
   - Path traversal prevention

3. **API Security**
   - CORS configuration
   - Rate limiting
   - Input validation
   - Error handling

## Testing

The backend uses Rust's built-in testing framework:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_user_registration() {
        // Test implementation
    }
}
```

## Deployment

1. **Requirements**
   - Rust toolchain
   - PostgreSQL database
   - File storage directory
   - Environment variables

2. **Build**
   ```bash
   cargo build --release
   ```

3. **Run**
   ```bash
   ./target/release/zipmind-backend
   ```

## Monitoring

The application includes logging and monitoring:

- Request logging
- Error tracking
- Performance metrics
- Health checks

## Contributing

See the [Contributing Guide](../contributing.md) for:

- Development setup
- Code style guidelines
- Testing requirements
- Pull request process

## Troubleshooting

Common issues and solutions:

1. **Database Connection**
   - Check connection string
   - Verify database is running
   - Check user permissions

2. **File Upload**
   - Verify storage directory permissions
   - Check file size limits
   - Validate file types

3. **Authentication**
   - Check JWT secret
   - Verify token expiration
   - Check password hashing 