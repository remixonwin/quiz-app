#!/bin/bash

# Set the DATABASE_URL environment variable
export DATABASE_URL="postgresql://remixonwin:600181@localhost/quiz_app"

# Drop and recreate the database
psql -U remixonwin -d postgres -c "DROP DATABASE IF EXISTS quiz_app;"
psql -U remixonwin -d postgres -c "CREATE DATABASE quiz_app;"

# Run the consolidated migration
psql -U remixonwin -d quiz_app -f migrations/20240119000003_consolidated_schema.sql
