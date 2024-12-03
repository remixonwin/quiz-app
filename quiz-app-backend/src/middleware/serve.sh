#!/bin/bash

# Constants
readonly FRONTEND_PORT=3000
readonly BACKEND_PORT=8080
readonly FRONTEND_DIR="quiz-app-frontend"
readonly BACKEND_DIR="quiz-app-backend"
readonly DATABASE_NAME="quiz_app"
readonly RELEASE_TARGET="target/release/quiz-app-backend"

# Utility Functions

# Check if port is in use and clean it
check_port() {
    local port=$1
    if lsof -ti:"$port" >/dev/null; then
        lsof -ti:"$port" | xargs kill -9 2>/dev/null || true
    fi
}

# Ensure the database exists
ensure_database() {
    if ! psql -lqt | cut -d \| -f 1 | grep -qw "$DATABASE_NAME" 2>/dev/null; then
        echo "Creating database $DATABASE_NAME..."
        createdb "$DATABASE_NAME"
    fi
}

# Check if backend needs rebuild
needs_rebuild() {
    local dir="$1"
    local target="$2"
    
    if [ ! -f "$target" ]; then
        return 0
    fi
    
    if find "$dir/src" -name "*.rs" -newer "$target" 2>/dev/null | grep -q .; then
        return 0
    fi
    
    if [ "$dir/Cargo.toml" -nt "$target" ]; then
        return 0
    fi
    
    return 1
}

# Main Serve Function
serve() {
    echo "Cleaning up existing processes..."
    check_port "$FRONTEND_PORT"
    check_port "$BACKEND_PORT"
    pkill -f "quiz-app-backend" || true
    pkill -f "react-scripts start" || true

    ensure_database

    # Setup and verify frontend (in background)
    cd "$FRONTEND_DIR"
    if [ ! -f "package.json" ]; then
        echo "Error: package.json not found in frontend directory"
        exit 1
    fi

    if [ ! -d "node_modules" ]; then
        echo "Installing frontend dependencies..."
        npm install --silent &
        INSTALL_PID=$!
    fi

    cd "../$BACKEND_DIR"
    
    if [ ! -f .env ]; then
        echo "Creating .env file..."
        cat > .env << EOL
DATABASE_URL=postgresql://postgres:postgres@localhost:5432/$DATABASE_NAME
JWT_SECRET=your_development_secret_key
RUST_LOG=debug
SERVER_PORT=$BACKEND_PORT
EOL
    fi

    export DATABASE_URL="postgresql://postgres:postgres@localhost:5432/$DATABASE_NAME"
    export JWT_SECRET="your_development_secret_key"
    export RUST_LOG="debug"
    export SERVER_PORT="$BACKEND_PORT"

    if needs_rebuild . "$RELEASE_TARGET"; then
        echo "Changes detected, rebuilding backend..."
        cargo check && cargo clippy && cargo build --release
    else
        echo "Using cached backend build..."
    fi

    echo "Starting backend..."
    "./$RELEASE_TARGET" &
    BACKEND_PID=$!

    if [ ! -z ${INSTALL_PID+x} ]; then
        echo "Waiting for frontend dependencies to finish installing..."
        wait $INSTALL_PID
    fi

    cd "../$FRONTEND_DIR"
    echo "Starting frontend..."
    PORT=$FRONTEND_PORT npm start &
    FRONTEND_PID=$!

    echo "Waiting for services to start..."
    for i in {1..30}; do
        BACKEND_READY=0
        FRONTEND_READY=0
        
        nc -z localhost "$BACKEND_PORT" 2>/dev/null && BACKEND_READY=1
        nc -z localhost "$FRONTEND_PORT" 2>/dev/null && FRONTEND_READY=1
        
        if [ $BACKEND_READY -eq 1 ] && [ $FRONTEND_READY -eq 1 ]; then
            echo "App is running!"
            echo "Frontend: http://localhost:$FRONTEND_PORT"
            echo "Backend: http://localhost:$BACKEND_PORT"
            break
        fi
        
        if [ $i -eq 30 ]; then
            echo "Error: Services failed to start"
            kill $FRONTEND_PID $BACKEND_PID 2>/dev/null || true
            exit 1
        fi
        sleep 1
    done

    trap 'kill $FRONTEND_PID $BACKEND_PID 2>/dev/null || true' INT
    wait
}

# Main command handler
case "${1:-serve}" in
    "serve"|"")
        serve
        ;;
    *)
        echo "Usage: $0 [serve]"
        exit 1
        ;;
esac

# Create symbolic link
ln -s /home/remixonwin/Documents/quiz-app/quiz-app-backend/src/middleware/serve.sh /usr/local/bin/serve-quiz-app