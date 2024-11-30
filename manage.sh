#!/bin/bash

# Constants
readonly FRONTEND_PORT=3000
readonly BACKEND_PORT=8080
readonly FRONTEND_DIR="quiz-app-frontend"
readonly BACKEND_DIR="quiz-app-backend"
readonly DATABASE_NAME="quiz_app"

# Color constants
readonly COLOR_GREEN='\033[0;32m'
readonly COLOR_BLUE='\033[0;34m'
readonly COLOR_RED='\033[0;31m'
readonly COLOR_YELLOW='\033[1;33m'
readonly COLOR_NC='\033[0m'

# Logging functions
log_info() { echo -e "${COLOR_BLUE}ℹ️  $1${COLOR_NC}"; }
log_success() { echo -e "${COLOR_GREEN}✅ $1${COLOR_NC}"; }
log_error() { echo -e "${COLOR_RED}❌ $1${COLOR_NC}"; }
log_warning() { echo -e "${COLOR_YELLOW}⚠️  $1${COLOR_NC}"; }

# Error handling
set -euo pipefail
trap 'log_error "An error occurred. Exiting..."; exit 1' ERR

# Utility functions
check_command() {
    local cmd=$1
    if ! command -v "$cmd" &> /dev/null; then
        log_error "$cmd is not installed. Please install it first."
        exit 1
    fi
}

check_port() {
    local port=$1
    if lsof -ti:"$port" >/dev/null; then
        log_warning "Port $port is in use. Cleaning up..."
        lsof -ti:"$port" | xargs kill -9
    fi
}

setup_database() {
    log_info "Setting up PostgreSQL database..."
    if psql -lqt | cut -d \| -f 1 | grep -qw "$DATABASE_NAME"; then
        log_info "Database already exists"
    else
        createdb "$DATABASE_NAME"
        log_success "Created database: $DATABASE_NAME"
    fi
}

setup_frontend() {
    log_info "Setting up Frontend..."
    cd "$FRONTEND_DIR"
    npm install
    cd ..
    log_success "Frontend dependencies installed"
}

setup_backend() {
    log_info "Setting up Backend..."
    cd "$BACKEND_DIR"
    
    # Create .env file if it doesn't exist
    if [ ! -f .env ]; then
        log_info "Creating .env file..."
        cat > .env << EOL
DATABASE_URL=postgresql://postgres:password@localhost:5432/$DATABASE_NAME
JWT_SECRET=development_secret_key
RUST_LOG=debug
SERVER_PORT=$BACKEND_PORT
EOL
        log_success "Created .env file"
    fi

    cd ..
    log_success "Backend setup completed"
}

run_tests() {
    local component=$1
    
    if [ "$component" = "backend" ] || [ "$component" = "all" ]; then
        log_info "Running backend tests..."
        cd "$BACKEND_DIR"
        cargo test
        cd ..
    fi
    
    if [ "$component" = "frontend" ] || [ "$component" = "all" ]; then
        log_info "Running frontend tests..."
        cd "$FRONTEND_DIR"
        CI=true npm test
        cd ..
    fi
}

start_servers() {
    # Clean up any existing processes
    check_port "$FRONTEND_PORT"
    check_port "$BACKEND_PORT"
    pkill -f "quiz-app-backend" || true
    pkill -f "react-scripts start" || true

    # Start backend
    log_info "Starting backend server..."
    cd "$BACKEND_DIR"
    cargo run &
    cd ..

    # Wait for backend to start
    sleep 5

    # Start frontend
    log_info "Starting frontend server..."
    cd "$FRONTEND_DIR"
    npm start &
    cd ..

    log_success "🚀 App is starting up!"
    log_info "📱 Frontend: http://localhost:$FRONTEND_PORT"
    log_info "🔧 Backend: http://localhost:$BACKEND_PORT"
}

setup_environment() {
    log_info "Checking required tools..."
    check_command "node"
    check_command "npm"
    check_command "cargo"
    check_command "psql"

    setup_database
    setup_frontend
    setup_backend
    log_success "Environment setup completed"
}

# Main execution
main() {
    local command=$1
    shift || true

    case $command in
        "setup")
            setup_environment
            ;;
        "start")
            start_servers
            ;;
        "test")
            local test_component=${1:-"all"}
            run_tests "$test_component"
            ;;
        "clean")
            check_port "$FRONTEND_PORT"
            check_port "$BACKEND_PORT"
            log_success "Cleaned up running processes"
            ;;
        *)
            log_error "Unknown command: $command"
            echo "Usage: $0 {setup|start|test [all|frontend|backend]|clean}"
            exit 1
            ;;
    esac
}

# Script entry point
if [ $# -eq 0 ]; then
    log_error "No command provided"
    echo "Usage: $0 {setup|start|test [all|frontend|backend]|clean}"
    exit 1
fi

main "$@"
