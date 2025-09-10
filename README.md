# GeminiPocket

AI image generation and editing powered by Google Gemini.

## Quick Start

```bash
# Install CLI
cargo install --path cli

# Generate an image
geminipocket-cli generate "a sunset over mountains"

# Edit an image
geminipocket-cli edit photo.png "add a rainbow"
```

## Features

- **Generate** - Create images from text prompts
- **Edit** - Transform existing images with AI
- **Fast** - Powered by Cloudflare Workers edge network
- **Simple** - Clean CLI with sensible defaults

## Project Structure

```
geminipocket/
├── backend/       # Cloudflare Worker API
├── cli/           # Command-line interface
└── (iOS, Android, Web coming soon)
```

## CLI Usage

```bash
# Show all commands
geminipocket-cli --help

# Generate with custom filename
geminipocket-cli generate "abstract art" --name my-art

# Configure output directory
geminipocket-cli config set output_dir ~/Pictures/AI

# Check API status
geminipocket-cli health
```

## API Endpoints

- **API Documentation**: https://geminipocket.guitaripod.workers.dev/ (Swagger UI)
- **OpenAPI Spec**: https://geminipocket.guitaripod.workers.dev/openapi

## Development

```bash
# Run worker locally
cd backend && wrangler dev

# Build CLI
cargo build --release -p geminipocket

# Install CLI globally
cargo install --path cli

# Build and deploy worker (from root)
./build.sh
```

## License

MIT
