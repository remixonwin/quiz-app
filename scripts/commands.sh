#!/bin/bash

# Constants
readonly FRONTEND_PORT=3000
readonly BACKEND_PORT=8080
readonly FRONTEND_DIR="quiz-app-frontend"
readonly BACKEND_DIR="quiz-app-backend"
readonly DATABASE_NAME="quiz_app"

# Check if port is in use and clean it
check_port() {
    local port=$1
    if lsof -ti:"$port" >/dev/null; then
        lsof -ti:"$port" | xargs kill -9 2>/dev/null || true
    fi
}

# Run backend tests
test_backend() {
    cd "$BACKEND_DIR"
    cargo test --quiet -- --test-threads=1
    cd ..
}

# Run frontend tests
test_frontend() {
    cd "$FRONTEND_DIR"
    CI=true npm test
    cd ..
}

# Check if backend needs rebuild
needs_rebuild() {
    local dir="$1"
    local target="$2"
    
    # If target doesn't exist, rebuild needed
    if [ ! -f "$target" ]; then
        return 0
    fi
    
    # Check if any Rust source files are newer than target
    if find "$dir/src" -name "*.rs" -newer "$target" 2>/dev/null | grep -q .; then
        return 0
    fi
    
    # Check if Cargo.toml is newer than target
    if [ "$dir/Cargo.toml" -nt "$target" ]; then
        return 0
    fi
    
    return 1
}

# Start the application servers
serve() {
    # Clean up any existing processes
    echo "Cleaning up existing processes..."
    check_port "$FRONTEND_PORT"
    check_port "$BACKEND_PORT"
    pkill -f "quiz-app-backend" || true
    pkill -f "react-scripts start" || true

    # Ensure database exists (only check once)
    if ! psql -lqt | cut -d \| -f 1 | grep -qw "$DATABASE_NAME" 2>/dev/null; then
        echo "Creating database $DATABASE_NAME..."
        createdb "$DATABASE_NAME"
    fi

    # Setup and verify frontend (in background)
    cd "$FRONTEND_DIR"
    if [ ! -f "package.json" ]; then
        echo "Error: package.json not found in frontend directory"
        exit 1
    fi

    # Start frontend in background early
    if [ ! -d "node_modules" ]; then
        echo "Installing frontend dependencies..."
        npm install --silent &
        INSTALL_PID=$!
    fi

    # Switch to backend while frontend installs
    cd "../$BACKEND_DIR"
    
    # Create .env file if it doesn't exist
    if [ ! -f .env ]; then
        echo "Creating .env file..."
        cat > .env << EOL
DATABASE_URL=postgresql://postgres:postgres@localhost:5432/$DATABASE_NAME
JWT_SECRET=your_development_secret_key
RUST_LOG=debug
SERVER_PORT=$BACKEND_PORT
EOL
    fi

    # Export environment variables
    export DATABASE_URL="postgresql://postgres:postgres@localhost:5432/$DATABASE_NAME"
    export JWT_SECRET="your_development_secret_key"
    export RUST_LOG="debug"
    export SERVER_PORT="$BACKEND_PORT"

    # Check if we need to rebuild
    RELEASE_TARGET="target/release/quiz-app-backend"
    if needs_rebuild . "$RELEASE_TARGET"; then
        echo "Changes detected, rebuilding backend..."
        
        # Run checks only if source files changed
        echo "Running cargo check..."
        if ! cargo check; then
            echo "cargo check failed. Please fix the errors before continuing."
            exit 1
        fi

        echo "Running cargo clippy..."
        if ! cargo clippy; then
            echo "cargo clippy found issues. Please fix them before continuing."
            exit 1
        fi

        # Build release version
        echo "Building backend..."
        cargo build --release
    else
        echo "Using cached backend build..."
    fi

    # Start backend
    echo "Starting backend..."
    "./$RELEASE_TARGET" &
    BACKEND_PID=$!
    cd ..

    # If we were installing frontend deps, wait for it
    if [ ! -z ${INSTALL_PID+x} ]; then
        echo "Waiting for frontend dependencies to finish installing..."
        wait $INSTALL_PID
    fi

    # Start frontend
    cd "$FRONTEND_DIR"
    echo "Starting frontend..."
    PORT=$FRONTEND_PORT npm start &
    FRONTEND_PID=$!
    cd ..

    # Wait for services to start (check both in parallel)
    echo "Waiting for services to start..."
    for i in {1..30}; do
        BACKEND_READY=0
        FRONTEND_READY=0
        
        nc -z localhost "$BACKEND_PORT" 2>/dev/null && BACKEND_READY=1
        nc -z localhost "$FRONTEND_PORT" 2>/dev/null && FRONTEND_READY=1
        
        if [ $BACKEND_READY -eq 1 ] && [ $FRONTEND_READY -eq 1 ]; then
            echo " App is running!"
            echo " Frontend: http://localhost:$FRONTEND_PORT"
            echo " Backend: http://localhost:$BACKEND_PORT"
            break
        fi
        
        if [ $i -eq 30 ]; then
            echo "Error: Services failed to start"
            kill $FRONTEND_PID $BACKEND_PID 2>/dev/null || true
            exit 1
        fi
        sleep 1
    done

    # Keep script running and handle cleanup on interrupt
    trap 'kill $FRONTEND_PID $BACKEND_PID 2>/dev/null || true' INT
    wait
}

# Main command handler
case "${1:-serve}" in
    "test")
        if [ "${2:-all}" = "frontend" ]; then
            test_frontend
        elif [ "${2:-all}" = "backend" ]; then
            test_backend
        else
            test_backend && test_frontend
        fi
        ;;
    "serve"|"")
        serve
        ;;
    *)
        echo "Usage: $0 [serve|test [frontend|backend]]"
        exit 1
        ;;
esac
