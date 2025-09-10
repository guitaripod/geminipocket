#!/bin/bash
set -e

echo "Building and deploying Gemini Pocket backend..."

cd backend

echo "Building worker..."
./build.sh

echo "Deploying to Cloudflare..."
npx wrangler deploy

echo "Deployment complete!"
