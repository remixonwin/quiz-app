#!/bin/bash

# Set the DATABASE_URL environment variable
export DATABASE_URL="postgresql://remixonwin:600181@localhost/quiz_app"

# Run the seeder
cargo run --bin seeder
