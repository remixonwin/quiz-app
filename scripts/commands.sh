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
    
    if [ "$component" = "backend" ]; then
        log_info "Running backend tests..."
        cd "$BACKEND_DIR"
        cargo test
        cd ..
    elif [ "$component" = "frontend" ]; then
        log_info "Running frontend tests..."
        cd "$FRONTEND_DIR"
        CI=true npm test
        cd ..
    else
        log_info "Running all tests..."
        cd "$BACKEND_DIR"
        cargo test
        cd ..
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

clean_processes() {
    check_port "$FRONTEND_PORT"
    check_port "$BACKEND_PORT"
    pkill -f "quiz-app-backend" || true
    pkill -f "react-scripts start" || true
    log_success "Cleaned up running processes"
}

check_network_ports() {
    log_info "Checking network ports availability..."
    local ports=($FRONTEND_PORT $BACKEND_PORT)
    for port in "${ports[@]}"; do
        if ! nc -z localhost $port; then
            log_success "Port $port is available"
        else
            log_warning "Port $port is in use, will be cleaned up during start"
        fi
    done
}

check_database_connection() {
    log_info "Checking database connection..."
    if psql -lqt | cut -d \| -f 1 | grep -qw "$DATABASE_NAME"; then
        if psql -d "$DATABASE_NAME" -c '\q' 2>/dev/null; then
            log_success "Database connection successful"
            return 0
        else
            log_error "Database exists but connection failed"
            return 1
        fi
    else
        log_warning "Database does not exist, will be created during setup"
        return 0
    fi
}

check_dependencies() {
    log_info "Checking project dependencies..."
    
    # Check backend dependencies
    cd "$BACKEND_DIR"
    if ! cargo check --quiet; then
        log_error "Backend dependencies check failed"
        cd ..
        return 1
    fi
    cd ..
    log_success "Backend dependencies OK"

    # Check frontend dependencies
    cd "$FRONTEND_DIR"
    if [ ! -d "node_modules" ] || [ ! -f "package-lock.json" ]; then
        log_warning "Frontend dependencies not installed, will be installed during setup"
    else
        if ! npm list >/dev/null 2>&1; then
            log_error "Frontend dependencies check failed"
            cd ..
            return 1
        fi
        log_success "Frontend dependencies OK"
    fi
    cd ..
    return 0
}

check_environment_files() {
    log_info "Checking environment files..."
    local missing_files=0

    if [ ! -f "$BACKEND_DIR/.env" ]; then
        log_warning "Backend .env file missing, will be created during setup"
        missing_files=1
    else
        log_success "Backend .env exists"
    fi

    return $missing_files
}

serve_application() {
    log_info "🔍 Starting comprehensive system check..."
    
    # Step 1: Check all system requirements
    check_command "node" || exit 1
    check_command "npm" || exit 1
    check_command "cargo" || exit 1
    check_command "psql" || exit 1
    check_command "nc" || exit 1
    log_success "✓ System requirements met"

    # Step 2: Check network ports
    check_network_ports
    log_success "✓ Network ports checked"

    # Step 3: Check database
    check_database_connection
    log_success "✓ Database check complete"

    # Step 4: Check project dependencies
    check_dependencies
    log_success "✓ Dependencies check complete"

    # Step 5: Check environment files
    check_environment_files
    log_success "✓ Environment files check complete"

    # Step 6: Run setup if any checks indicated missing components
    if [ ! -d "$FRONTEND_DIR/node_modules" ] || [ ! -f "$BACKEND_DIR/.env" ]; then
        log_info "🔧 Some components missing, running setup..."
        setup_environment
    fi

    # Step 7: Run a quick test to ensure everything is working
    log_info "🧪 Running quick system test..."
    cd "$BACKEND_DIR"
    if ! cargo test --quiet -- --test-threads=1 2>/dev/null; then
        log_error "Backend tests failed"
        cd ..
        exit 1
    fi
    cd ..
    log_success "✓ Quick system test passed"

    # Step 8: Start the application
    log_success "🚀 All checks passed! Starting application..."
    start_servers
}

# Get the command from first argument
if [ $# -eq 0 ]; then
    log_error "No command provided"
    echo "Available commands: setup, start, test [frontend|backend], clean, serve"
    exit 1
fi

COMMAND="$1"
shift  # Remove first argument, leaving remaining args

case $COMMAND in
    "setup")
        setup_environment
        ;;
    "start")
        start_servers
        ;;
    "test")
        if [ $# -eq 0 ]; then
            run_tests "all"
        else
            run_tests "$1"
        fi
        ;;
    "clean")
        clean_processes
        ;;
    "serve")
        serve_application
        ;;
    *)
        log_error "Unknown command: $COMMAND"
        echo "Available commands: setup, start, test [frontend|backend], clean, serve"
        exit 1
        ;;
esac
