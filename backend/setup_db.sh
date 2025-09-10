#!/bin/bash
set -e

echo "Setting up GeminiPocket D1 database..."

# Create D1 database
echo "Creating D1 database..."
wrangler d1 create geminipocket

# Get the database ID and update wrangler.toml
DB_ID=$(wrangler d1 list | grep geminipocket | awk '{print $1}')

if [ -n "$DB_ID" ]; then
    echo "Database ID: $DB_ID"
    sed -i "s/database_id = \"geminipocket-db\"/database_id = \"$DB_ID\"/" wrangler.toml
else
    echo "Failed to get database ID"
    exit 1
fi

# Run migrations
echo "Running database migrations..."
wrangler d1 execute geminipocket --file=migrations/001_create_users.sql

echo "Database setup complete!"
echo "Database ID: $DB_ID"