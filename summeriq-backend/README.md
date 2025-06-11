# SummerIQ Backend

A robust file upload service built with Rust, Actix-web, and PostgreSQL.

## Features

- File upload with multipart form support
- Special handling for ZIP files (automatic extraction)
- File metadata storage in PostgreSQL
- Secure file storage with unique filenames
- File retrieval by ID
- CORS support for frontend integration

## Prerequisites

- Rust (latest stable version)
- PostgreSQL
- Cargo

## Setup

1. Create a PostgreSQL database:
```sql
CREATE DATABASE summeriq;
```

2. Copy the `.env.example` file to `.env` and update the values:
```bash
cp .env.example .env
```

3. Install dependencies and build:
```bash
cargo build
```

4. Run the migrations:
```bash
cargo sqlx migrate run
```

5. Start the server:
```bash
cargo run
```

## API Endpoints

### Upload File
```
POST /upload
Content-Type: multipart/form-data

file: <file>
```

### Get File
```
GET /files/{file_id}
```

## Testing

You can test the file upload endpoint using curl:

```bash
# Upload a file
curl -X POST -F "file=@/path/to/your/file.txt" http://localhost:8080/upload

# Upload a ZIP file
curl -X POST -F "file=@/path/to/your/archive.zip" http://localhost:8080/upload

# Get a file
curl http://localhost:8080/files/{file_id}
```

## Environment Variables

- `DATABASE_URL`: PostgreSQL connection string
- `PORT`: Server port (default: 8080)
- `UPLOAD_DIR`: Directory for file storage
- `RUST_LOG`: Logging level (default: debug) 