serve() {
    echo "Cleaning up existing processes..."
    check_port "$FRONTEND_PORT"
    check_port "$BACKEND_PORT"
    pkill -f "quiz-app-backend|react-scripts start" || true

    if ! psql -lqt | cut -d \| -f 1 | grep -qw "$DATABASE_NAME" 2>/dev/null; then
        echo "Creating database $DATABASE_NAME..."
        createdb "$DATABASE_NAME"
    fi

    cd "$FRONTEND_DIR"
    if [ ! -f "package.json" ]; then
        echo "Error: package.json not found in frontend directory"
        exit 1
    fi
    # Continue with the rest of the setup...
}