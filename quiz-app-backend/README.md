# Quiz App Backend

A Rust-based backend for a quiz application using Actix-web and PostgreSQL.

## Features

- User authentication and authorization
- CRUD operations for quizzes
- Question and answer management
- Quiz attempt tracking and scoring
- Error handling with custom error types
- Database migrations with SQLx

## Prerequisites

- Rust (latest stable version)
- PostgreSQL
- SQLx CLI

## Setup

1. Install dependencies:
   ```bash
   cargo install sqlx-cli
   ```

2. Set up the database:
   ```bash
   # Create the database
   sqlx database create
   
   # Run migrations
   sqlx migrate run
   ```

3. Run the server:
   ```bash
   cargo run
   ```

## API Endpoints

### Quizzes
- `GET /quizzes` - List all quizzes
- `POST /quizzes` - Create a new quiz
- `GET /quizzes/{id}` - Get a specific quiz
- `PUT /quizzes/{id}` - Update a quiz
- `DELETE /quizzes/{id}` - Delete a quiz

### Quiz Attempts
- `POST /quizzes/{id}/submit` - Submit a quiz attempt

## Error Handling

The application uses a custom `AppError` type that handles:
- Database errors
- Authentication errors
- Not found errors
- Bad request errors

Errors are returned as JSON with appropriate HTTP status codes.

## Running Endpoint Tests

To execute endpoint tests within a Docker container, follow these steps:

1. **Build the test image:**
   ```bash
   docker build --target endpoint-tester -t quiz-app-endpoint-tests .
   ```

2. **Run the tests:**
   ```bash
   docker run --rm --network host quiz-app-endpoint-tests
   ```

   *Ensure that the PostgreSQL database is running and accessible at the `DATABASE_URL` specified in the Dockerfile.*
