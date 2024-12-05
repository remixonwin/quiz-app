#!/bin/bash
set -e

# Change to the script's directory
cd "$(dirname "$0")"

# Wait for Postgres to be ready using localhost
echo "Waiting for Postgres..."
while ! pg_isready -U postgres -h localhost -p 5432 > /dev/null 2>&1; do
    echo "Postgres is not ready yet..."
    sleep 1
done

echo "Postgres is ready!"

# Set database URL to match docker-compose credentials
export DATABASE_URL=postgres://remixonwin:600181@localhost:5432/quiz_app

# Add cargo to PATH
if [ "$SUDO_USER" ]; then
    export PATH="/home/$SUDO_USER/.cargo/bin:$PATH"
else
    export PATH="$HOME/.cargo/bin:$PATH"
fi

# Check if cargo is installed
if ! command -v cargo &> /dev/null; then
    echo "cargo could not be found. Please install Rust from https://www.rust-lang.org/tools/install"
    exit 1
fi

# Set default Rust toolchain
rustup default stable

# Drop and recreate the database to reset migrations
dropdb -h localhost -U remixonwin quiz_app || true
createdb -h localhost -U remixonwin quiz_app

# Run migrations and prepare SQLx
cargo sqlx migrate run
cargo sqlx prepare --workspace

# Function to pull Docker images with retries
pull_image() {
    local image=$1
    local retries=3
    local count=0
    until [ $count -ge $retries ]
    do
        docker pull "$image" && break
        count=$((count+1))
        echo "Retrying to pull $image ($count/$retries)..."
        sleep 5
    done
    if [ $count -ge $retries ]; then
        echo "Failed to pull $image after $retries attempts."
        exit 1
    fi
}

# Stop and remove any running containers using postgres:13
docker ps -q --filter ancestor=postgres:13 | xargs -r docker stop
docker ps -aq --filter ancestor=postgres:13 | xargs -r docker rm

# Pull required Docker images
pull_image debian:bullseye-slim
pull_image node:18.17.1-slim
pull_image rust:1.70.0

# Remove older Docker images and containers
docker-compose down --rmi all

# Build the Docker image
cd ..
docker build -t quiz-app:latest .

# Start Docker containers
docker-compose up -d

# Check running containers
docker-compose ps

# View logs
docker-compose logs -f

# Access the application
echo "Access the application at http://localhost:3000"