# GeminiPocket

AI image and video generation and editing powered by Google Gemini and Veo.

## 🚀 Quick Start

<details>
<summary>Click to expand quick start guide</summary>

```bash
# Install CLI
cargo install --path cli

# Generate an image
geminipocket-cli generate "a sunset over mountains"

# Edit an image
geminipocket-cli edit photo.png "add a rainbow"
```

</details>

## ✨ Features

- **Generate Images** - Create images from text prompts
- **Edit Images** - Transform existing images with AI
- **Generate Videos** - Create videos from text prompts
- **Edit Videos** - Transform images into videos with AI
- **Web Interface** - User-friendly web app with drag-and-drop
- **Fast** - Powered by Cloudflare Workers edge network
- **Simple** - Clean CLI with sensible defaults

## 📁 Project Structure

<details>
<summary>Click to view project structure</summary>

```
geminipocket/
├── backend/       # Cloudflare Worker API
├── cli/           # Command-line interface
├── web/           # Web interface (static files)
└── (iOS, Android coming soon)
```

</details>

## 🌐 Web Interface

<details>
<summary>Click to learn about the web interface</summary>

**Access the web interface at: [https://geminipocket.guitaripod.workers.dev](https://geminipocket.guitaripod.workers.dev)**

### Features:
- Generate images from text prompts
- Edit existing images with drag-and-drop
- Generate videos from text prompts
- Edit videos from images with AI
- Responsive design with dark mode support
- Real-time API health monitoring
- Direct access to API documentation

</details>

## 💻 CLI Usage

<details>
<summary>Click to view CLI commands</summary>

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

</details>

## 🔌 API Endpoints

<details>
<summary>Click to view API endpoints</summary>

- **API Documentation**: [https://geminipocket.guitaripod.workers.dev/docs](https://geminipocket.guitaripod.workers.dev/docs) (Swagger UI)
- **OpenAPI Spec**: [https://geminipocket.guitaripod.workers.dev/openapi](https://geminipocket.guitaripod.workers.dev/openapi)

</details>

## 🛠️ Development

<details>
<summary>Click to view development setup</summary>

### Backend (Cloudflare Worker)
```bash
# Run worker locally
cd backend && wrangler dev

# Build and deploy worker (from root)
./build_deploy.sh
```

### CLI
```bash
# Build CLI
cargo build --release -p geminipocket

# Install CLI globally
cargo install --path cli
```

### Web Interface
```bash
# Install dependencies
cd web && npm install

# Start development server
npm run dev

# Build for production (static files)
npm run build
```

</details>

## 📄 License

MIT
