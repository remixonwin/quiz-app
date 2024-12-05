# Use a multi-stage build to optimize the final image size

# Stage 1: Build frontend
FROM node:18.17.1-slim AS frontend-builder
WORKDIR /app/frontend
COPY quiz-app-frontend/package*.json ./
RUN npm install
COPY quiz-app-frontend/ .
ENV REACT_APP_API_BASE_URL=http://localhost:8080/api
RUN npm run build

# Stage 2: Build the backend
FROM rust:1.83.0 AS backend-builder
WORKDIR /app/quiz-app-backend

# Create the cargo user before changing ownership
RUN useradd -m -u 1000 -U cargo && \
    apt-get update && \
    apt-get install -y pkg-config libssl-dev netcat-openbsd && \
    cargo install sqlx-cli --no-default-features --features postgres --locked && \
    mkdir -p /usr/local/cargo/registry && \
    chmod -R 755 /usr/local/cargo/registry && \
    chown -R cargo:cargo /usr/local/cargo/registry && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

# Create cargo user and set permissions
RUN chown -R cargo:cargo /app && \
    mkdir -p /usr/local/cargo/registry/cache && \
    mkdir -p /usr/local/cargo/registry/index && \
    chown -R cargo:cargo /usr/local/cargo/registry/cache && \
    chown -R cargo:cargo /usr/local/cargo/registry/index

USER cargo

# Copy backend dependency files and fetch dependencies
COPY --chown=cargo:cargo quiz-app-backend/Cargo.toml quiz-app-backend/Cargo.lock ./
RUN cargo fetch

# Copy backend files and SQLx data
COPY --chown=cargo:cargo quiz-app-backend/ .
COPY --chown=cargo:cargo migrations /app/migrations
COPY --chown=cargo:cargo quiz-app-backend/.sqlx /app/.sqlx

# Build with prepared queries
ENV SQLX_OFFLINE=true
RUN cargo build --release

# Stage 3: Final image
FROM debian:bullseye-slim
WORKDIR /app

# Install nginx for serving frontend
RUN apt-get update && \
    apt-get install -y nginx libssl-dev ca-certificates netcat-openbsd && \
    rm -rf /var/lib/apt/lists/*

# Copy nginx configuration
COPY nginx.conf /etc/nginx/nginx.conf

# Copy frontend build
COPY --from=frontend-builder /app/frontend/build /app/frontend

# Copy backend binary
COPY --from=backend-builder /app/quiz-app-backend/target/release/quiz-app-backend /app/backend/

# Copy wait script
COPY wait-for-it.sh /app/backend/wait-for-it.sh
RUN useradd -m -u 1000 -U app && \
    chmod +x /app/backend/wait-for-it.sh && chown app:app /app/backend/wait-for-it.sh

# Create non-root user
RUN chown -R app:app /app

USER app

# Set environment variables
ENV RUST_LOG=info
ENV SKIP_DB_INIT=

EXPOSE 80 8080

# Start both nginx and backend
CMD ["sh", "-c", "./backend/wait-for-it.sh db:5432 -- ./backend/quiz-app-backend & nginx -g 'daemon off;'"]

# Stage 4: Run Endpoint Tests
FROM backend-builder AS endpoint-tester
WORKDIR /app/quiz-app-backend

# Set the CARGO_HOME environment variable to the cargo user's home directory
ENV CARGO_HOME=/home/cargo/.cargo

# Ensure the CARGO_HOME directory exists and is owned by the cargo user
RUN mkdir -p $CARGO_HOME && \
    chmod -R 755 $CARGO_HOME

USER cargo

# Install SQLx CLI for running migrations during tests
RUN cargo install sqlx-cli --no-default-features --features postgres --locked

# Copy necessary files for testing
COPY --from=backend-builder /app/quiz-app-backend/ ./ 
COPY --from=backend-builder /app/migrations ./migrations

# Set environment variables for testing
ENV DATABASE_URL=postgres://remixonwin:600181@db/quiz_app_test
ENV SQLX_OFFLINE=false

# Execute endpoint tests
CMD ["cargo", "test", "--", "endpoint_tests"]