# API Documentation

## Overview

The ZipMind API provides endpoints for user authentication, file management, and AI-powered code analysis. This documentation details all available endpoints, their request/response formats, and authentication requirements.

## Base URL

```
http://localhost:8080/api
```

## Authentication

All endpoints except registration and login require a JWT token in the Authorization header:

```http
Authorization: Bearer <jwt_token>
```

## Endpoints

### Authentication

#### Register User

```http
POST /auth/register
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "securepassword"
}
```

**Response (201 Created)**
```json
{
  "id": "uuid",
  "email": "user@example.com",
  "created_at": "2024-03-20T12:00:00Z"
}
```

#### Login

```http
POST /auth/login
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "securepassword"
}
```

**Response (200 OK)**
```json
{
  "token": "jwt_token",
  "user": {
    "id": "uuid",
    "email": "user@example.com"
  }
}
```

### File Management

#### Upload File

```http
POST /upload
Authorization: Bearer <jwt_token>
Content-Type: multipart/form-data

file: <file_data>
```

**Response (201 Created)**
```json
{
  "id": "uuid",
  "filename": "original_filename.zip",
  "file_size": 1024,
  "file_type": "application/zip",
  "created_at": "2024-03-20T12:00:00Z"
}
```

#### Get File Info

```http
GET /files/{file_id}
Authorization: Bearer <jwt_token>
```

**Response (200 OK)**
```json
{
  "id": "uuid",
  "filename": "original_filename.zip",
  "file_size": 1024,
  "file_type": "application/zip",
  "created_at": "2024-03-20T12:00:00Z",
  "analysis_status": "completed"
}
```

#### List User Files

```http
GET /files
Authorization: Bearer <jwt_token>
```

**Response (200 OK)**
```json
{
  "files": [
    {
      "id": "uuid",
      "filename": "original_filename.zip",
      "file_size": 1024,
      "file_type": "application/zip",
      "created_at": "2024-03-20T12:00:00Z"
    }
  ]
}
```

### AI Analysis

#### Start Analysis

```http
POST /analysis/{file_id}
Authorization: Bearer <jwt_token>
```

**Response (202 Accepted)**
```json
{
  "analysis_id": "uuid",
  "status": "started",
  "estimated_completion": "2024-03-20T12:05:00Z"
}
```

#### Get Analysis Status

```http
GET /analysis/{file_id}
Authorization: Bearer <jwt_token>
```

**Response (200 OK)**
```json
{
  "status": "completed",
  "results": {
    "code_structure": {
      "total_files": 10,
      "languages": ["JavaScript", "TypeScript"],
      "complexity_score": 0.75
    },
    "security_issues": [
      {
        "severity": "high",
        "description": "Potential SQL injection vulnerability",
        "location": "src/database.js:45"
      }
    ],
    "best_practices": [
      {
        "category": "security",
        "suggestion": "Use parameterized queries",
        "location": "src/database.js:45"
      }
    ]
  }
}
```

### Chat

#### Send Message

```http
POST /chat
Authorization: Bearer <jwt_token>
Content-Type: application/json

{
  "file_id": "uuid",
  "message": "What does this code do?"
}
```

**Response (200 OK)**
```json
{
  "response": "This code implements a user authentication system...",
  "references": [
    {
      "file": "src/auth.js",
      "line": 45,
      "context": "function authenticateUser() {"
    }
  ]
}
```

#### Get Chat History

```http
GET /chat/{file_id}
Authorization: Bearer <jwt_token>
```

**Response (200 OK)**
```json
{
  "messages": [
    {
      "id": "uuid",
      "role": "user",
      "content": "What does this code do?",
      "timestamp": "2024-03-20T12:00:00Z"
    },
    {
      "id": "uuid",
      "role": "assistant",
      "content": "This code implements a user authentication system...",
      "timestamp": "2024-03-20T12:00:01Z"
    }
  ]
}
```

## Error Responses

All endpoints may return the following error responses:

### 400 Bad Request
```json
{
  "error": "Invalid request parameters",
  "details": "Email must be a valid email address"
}
```

### 401 Unauthorized
```json
{
  "error": "Unauthorized",
  "details": "Invalid or expired token"
}
```

### 403 Forbidden
```json
{
  "error": "Forbidden",
  "details": "Insufficient permissions"
}
```

### 404 Not Found
```json
{
  "error": "Not Found",
  "details": "File not found"
}
```

### 500 Internal Server Error
```json
{
  "error": "Internal Server Error",
  "details": "An unexpected error occurred"
}
```

## Rate Limiting

API requests are limited to:
- 100 requests per minute for authenticated users
- 20 requests per minute for unauthenticated users

Rate limit headers are included in all responses:
```http
X-RateLimit-Limit: 100
X-RateLimit-Remaining: 99
X-RateLimit-Reset: 1616248800
```

## WebSocket API

### Connection

```javascript
const ws = new WebSocket('ws://localhost:8080/api/ws');
```

### Authentication

Send authentication message after connection:
```javascript
ws.send(JSON.stringify({
  type: 'auth',
  token: 'jwt_token'
}));
```

### Real-time Updates

The WebSocket connection provides real-time updates for:
- File analysis progress
- Chat messages
- System notifications

### Message Types

1. **Analysis Progress**
```json
{
  "type": "analysis_progress",
  "file_id": "uuid",
  "progress": 75,
  "status": "processing"
}
```

2. **Chat Message**
```json
{
  "type": "chat_message",
  "file_id": "uuid",
  "message": {
    "role": "assistant",
    "content": "Analysis complete..."
  }
}
```

3. **System Notification**
```json
{
  "type": "notification",
  "level": "info",
  "message": "File upload complete"
}
```

## Versioning

The API version is included in the URL path:
```
http://localhost:8080/api/v1/...
```

Current version: v1

## Support

For API support:
- Email: api-support@zipmind.com
- Documentation: https://docs.zipmind.com
- Status: https://status.zipmind.com 