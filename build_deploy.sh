#!/bin/bash
set -e

echo "Building and deploying Gemini Pocket backend..."

# Change to backend directory
cd backend

# Build the worker
echo "Building worker..."
./build.sh

# Deploy using wrangler
echo "Deploying to Cloudflare..."
npx wrangler deploy

echo "Deployment complete!"
