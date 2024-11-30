# Quiz App

A full-stack quiz application built with React and Rust.

## Quick Start

To use the Quiz App commands, first activate the development environment:

```bash
source scripts/activate
```

Once activated, you can use the following commands directly (without ./):

```bash
# Comprehensive check and start (recommended)
serve           # Checks everything, sets up if needed, and starts the app

# Individual commands
setup           # Set up the environment
start           # Start the application
test            # Run all tests
test backend    # Run backend tests only
test frontend   # Run frontend tests only
clean           # Clean up processes
```

The `serve` command performs the following checks before starting the application:
1. ✓ System requirements (node, npm, cargo, postgresql)
2. ✓ Network ports availability
3. ✓ Database connection
4. ✓ Project dependencies
5. ✓ Environment files
6. ✓ Quick system test

If any components are missing, it will automatically run setup before starting.

## Project Structure

```
quiz-app/
├── quiz-app-frontend/     # React frontend
├── quiz-app-backend/      # Rust/Actix-web backend
└── quiz-app-database/     # PostgreSQL database schemas
```

## Prerequisites

- Node.js (v14 or later)
- Rust (latest stable)
- PostgreSQL (v13 or later)

## Setup Instructions

### Frontend Setup

1. Navigate to the frontend directory:
   ```bash
   cd quiz-app-frontend
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

3. Start the development server:
   ```bash
   npm start
   ```

### Backend Setup

1. Navigate to the backend directory:
   ```bash
   cd quiz-app-backend
   ```

2. Create a `.env` file with the following content:
   ```
   DATABASE_URL=postgres://username:password@localhost/quiz_app
   RUST_LOG=debug
   ```

3. Build and run the server:
   ```bash
   cargo run
   ```

### Database Setup

1. Create a PostgreSQL database:
   ```bash
   createdb quiz_app
   ```

2. Apply the schema:
   ```bash
   psql quiz_app < quiz-app-database/schema.sql
   ```

## Development Phases

1. Frontend Development (2 weeks)
2. Backend Development (4 weeks)
3. API Integration (1 week)
4. Database Integration (1 week)
5. Testing and Debugging (2 weeks)
6. Deployment (1 week)

## Security Measures

- HTTPS encryption
- Input validation
- Secure password practices
- Rate limiting

## Scalability Strategy

- Load balancing
- Caching
- CDN utilization

## API Documentation

API documentation will be available at `http://localhost:8080/docs` when running the backend server.
