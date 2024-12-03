#!/bin/bash

# Load the serve script
source ./serve.sh

# Mock functions and variables
export FRONTEND_PORT=3000
export BACKEND_PORT=8080
export FRONTEND_DIR="mock-frontend"
export BACKEND_DIR="mock-backend"
export DATABASE_NAME="mock_quiz_app"
export RELEASE_TARGET="mock-backend/target/release/quiz-app-backend"

# Create mock directories and files
mkdir -p "$FRONTEND_DIR"
mkdir -p "$BACKEND_DIR/target/release"
touch "$FRONTEND_DIR/package.json"
touch "$RELEASE_TARGET"

# Mock commands
mock_psql() {
    echo "Mock psql called"
}
mock_createdb() {
    echo "Mock createdb called"
}
mock_npm_install() {
    echo "Mock npm install called"
}
mock_npm_start() {
    echo "Mock npm start called"
}
mock_cargo_check() {
    echo "Mock cargo check called"
}
mock_cargo_clippy() {
    echo "Mock cargo clippy called"
}
mock_cargo_build() {
    echo "Mock cargo build called"
}
mock_cargo_run() {
    echo "Mock cargo run called"
}

# Override commands with mocks
alias psql=mock_psql
alias createdb=mock_createdb
alias npm=mock_npm
alias cargo=mock_cargo

# Mock npm command
mock_npm() {
    case "$1" in
        install)
            mock_npm_install
            ;;
        start)
            mock_npm_start
            ;;
        *)
            echo "Unknown npm command: $1"
            ;;
    esac
}

# Mock cargo command
mock_cargo() {
    case "$1" in
        check)
            mock_cargo_check
            ;;
        clippy)
            mock_cargo_clippy
            ;;
        build)
            mock_cargo_build
            ;;
        run)
            mock_cargo_run
            ;;
        *)
            echo "Unknown cargo command: $1"
            ;;
    esac
}

# Run the serve function and capture the output
output=$(serve)

# Check if the expected output is produced
if ! grep -q "Mock psql called" <<< "$output"; then
    echo "Test failed: psql was not called"
    exit 1
fi

if ! grep -q "Mock createdb called" <<< "$output"; then
    echo "Test failed: createdb was not called"
    exit 1
fi

if ! grep -q "Mock npm install called" <<< "$output"; then
    echo "Test failed: npm install was not called"
    exit 1
fi

if ! grep -q "Mock npm start called" <<< "$output"; then
    echo "Test failed: npm start was not called"
    exit 1
fi

if ! grep -q "Mock cargo check called" <<< "$output"; then
    echo "Test failed: cargo check was not called"
    exit 1
fi

if ! grep -q "Mock cargo clippy called" <<< "$output"; then
    echo "Test failed: cargo clippy was not called"
    exit 1
fi

if ! grep -q "Mock cargo build called" <<< "$output"; then
    echo "Test failed: cargo build was not called"
    exit 1
fi

if ! grep -q "Mock cargo run called" <<< "$output"; then
    echo "Test failed: cargo run was not called"
    exit 1
fi

echo "All tests passed"