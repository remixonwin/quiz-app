# Use a multi-stage build to optimize the final image size

# Stage 1: Build the frontend
FROM node:18.17.1-slim AS frontend-builder
WORKDIR /app/quiz-app-frontend
COPY quiz-app-frontend/package*.json ./
RUN npm install --legacy-peer-deps
COPY quiz-app-frontend .
RUN npm run build

# Stage 2: Build the backend
FROM rust:latest AS backend-builder
WORKDIR /app/quiz-app-backend

# Install build dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Ensure Rust and Cargo are up to date
RUN rustup update stable

# Verify Cargo version
RUN cargo --version

# Copy backend files
COPY quiz-app-backend/ .

# Build the backend
RUN cargo build --release

# Stage 3: Final stage
FROM node:18.17.1-slim
WORKDIR /app

# Copy frontend build
COPY --from=frontend-builder /app/quiz-app-frontend/build ./quiz-app-frontend/build

# Copy backend build
COPY --from=backend-builder /app/quiz-app-backend/target/release/quiz-app-backend ./quiz-app-backend

# Copy other necessary files
COPY quiz-app-backend/.env ./quiz-app-backend/.env

# Expose ports
EXPOSE 3000
EXPOSE 8080

# Start the application
CMD ["./quiz-app-backend", "serve"]