#!/bin/sh
set -e

host="$1"
shift
cmd="$@"

# Consider using dockerize instead
# Example CMD in docker-compose.yml:
# CMD ["dockerize", "-wait", "tcp://db:5432", "-timeout", "60s", "./backend/quiz-app-backend"]

# If retaining wait-for-it.sh, ensure robust error handling

until nc -z -v -w30 "$host" 5432; do
  echo "Waiting for database connection..."
  sleep 2
done

echo "Database is up - executing command"
exec $cmd