# GeminiPocket

AI image and video generation and editing powered by Google Gemini and Veo.

GeminiPocket is a comprehensive AI-powered tool for generating and editing images and videos using Google's latest AI models. It provides both a user-friendly web interface and a powerful command-line interface.

### Current Status ✅
- ✅ Image generation and editing (Gemini 2.5)
- ✅ Video generation and editing (Veo 3.0)
- ✅ Web interface with real-time progress
- ✅ CLI tool with full feature set
- ✅ RESTful API with OpenAPI documentation
- ✅ Cloudflare Workers deployment
- ✅ Authentication and user management
- ✅ Rate limiting and error handling

### Roadmap 🚧
- 🔄 Mobile apps (iOS/Android)
- 🔄 Batch processing capabilities
- 🔄 Advanced video editing features
- 🔄 Custom model fine-tuning
- 🔄 Integration with other AI services

## 🚀 Quick Start

### Web Interface (Easiest)
Visit [https://geminipocket.guitaripod.workers.dev](https://geminipocket.guitaripod.workers.dev) and start generating immediately!

### CLI Installation

<details>
<summary>Click to expand CLI setup guide</summary>

```bash
# Install CLI
cargo install --path cli

# Or build from source
git clone https://github.com/yourusername/geminipocket.git
cd geminipocket
cargo build --release --workspace
cargo install --path cli

# Verify installation
geminipocket-cli --help
```

</details>

### Quick Examples

```bash
# Generate a beautiful landscape
geminipocket-cli generate "majestic mountain landscape at sunset"

# Edit a photo to add effects
geminipocket-cli edit myphoto.jpg "add cinematic lighting and golden hour effect"

# Create a video from text
geminipocket-cli generate-video "time-lapse of a flower blooming"

# Transform image into animated video
geminipocket-cli edit-video drawing.png "bring this sketch to life with animation"
```

## ✨ Features

### 🎨 Image Generation & Editing
- **Generate Images** - Create stunning images from text prompts using Gemini 2.5
- **Edit Images** - Transform existing images with AI-powered editing
- **High Quality** - 1024x1024 resolution PNG images
- **Multiple Formats** - Support for PNG, JPG, GIF, WebP input/output

### 🎬 Video Generation & Editing
- **Generate Videos** - Create 8-second videos from text prompts using Veo 3.0
- **Edit Videos** - Transform images into animated videos
- **Multiple Resolutions** - 720p and 1080p video output
- **Aspect Ratios** - Support for 16:9 and 9:16 aspect ratios
- **Real-time Status** - Live progress updates during generation

### 🖥️ User Interfaces
- **Web Interface** - User-friendly web app with drag-and-drop
- **CLI Tool** - Powerful command-line interface with scripting support
- **API Access** - RESTful API with OpenAPI documentation

### ⚡ Performance & Reliability
- **Fast** - Powered by Cloudflare Workers edge network
- **Global CDN** - Low-latency access worldwide
- **Rate Limiting** - Built-in protection against abuse
- **Error Handling** - Comprehensive error reporting and recovery

## 📁 Project Structure

<details>
<summary>Click to view project structure</summary>

```
geminipocket/
├── backend/           # Cloudflare Worker API backend
│   ├── worker/        # Main Rust worker code
│   │   ├── src/       # Source files
│   │   │   ├── lib.rs         # Main API logic
│   │   │   ├── openapi.rs     # OpenAPI spec generation
│   │   │   └── swagger_ui.rs  # Swagger UI serving
│   │   ├── migrations/        # D1 database migrations
│   │   └── Cargo.toml         # Worker dependencies
│   └── wrangler.toml  # Cloudflare Worker config
├── cli/               # Command-line interface
│   ├── src/
│   │   ├── commands/  # CLI command implementations
│   │   ├── main.rs    # CLI entry point
│   │   └── types.rs   # Shared types
│   └── Cargo.toml     # CLI dependencies
├── web/               # Web interface (static files)
│   ├── public/        # Static web assets
│   │   ├── index.html # Main HTML page
│   │   ├── app.js     # Frontend JavaScript
│   │   └── styles.css # CSS styling
│   └── package.json   # Web dependencies
├── build_deploy.sh    # Build and deploy script
├── openapi-spec.json  # API specification
└── Cargo.toml         # Workspace configuration
```

</details>

## 🌐 Web Interface

**Access the web interface at: [https://geminipocket.guitaripod.workers.dev](https://geminipocket.guitaripod.workers.dev)**

### Features:
- **Image Generation** - Create images from text prompts
- **Image Editing** - Upload and edit existing images with drag-and-drop
- **Video Generation** - Generate videos from text descriptions
- **Video Editing** - Transform images into videos with AI
- **Real-time Progress** - Live status updates during generation
- **Responsive Design** - Works on desktop and mobile devices
- **API Health Monitoring** - Check service status
- **API Documentation** - Direct access to Swagger UI

### Usage:
1. Visit the web interface
2. Choose between image or video generation
3. Enter your prompt and adjust settings
4. Click generate and wait for completion
5. Download your generated content

## 💻 CLI Usage

### Installation
```bash
cargo install --path cli
```

### Basic Commands

```bash
# Show help and all available commands
geminipocket-cli --help

# Check API health
geminipocket-cli health

# Show API information
geminipocket-cli info
```

### Authentication

```bash
# Register a new account
geminipocket-cli auth register

# Login to existing account
geminipocket-cli auth login

# Check authentication status
geminipocket-cli auth status

# Logout and clear credentials
geminipocket-cli auth logout
```

### Image Generation

```bash
# Generate image from text prompt
geminipocket-cli generate "a sunset over mountains"

# Generate with custom filename
geminipocket-cli generate "abstract art" --name my-art

# Save to current directory
geminipocket-cli generate "landscape" --save
```

### Image Editing

```bash
# Edit existing image
geminipocket-cli edit photo.png "add a rainbow"

# Edit with custom output name
geminipocket-cli edit image.jpg "make it black and white" --name bw-version

# Save to current directory
geminipocket-cli edit photo.png "enhance colors" --save
```

### Video Generation

```bash
# Generate video from text prompt
geminipocket-cli generate-video "drone shot following a car along coastal road"

# Generate with custom settings
geminipocket-cli generate-video "majestic lion in savannah" \
  --aspect-ratio 9:16 \
  --resolution 1080p \
  --negative-prompt "blurry, low quality"

# Generate with custom filename
geminipocket-cli generate-video "ocean waves" --name waves-video
```

### Video Editing

```bash
# Transform image into video
geminipocket-cli edit-video photo.png "make it dance and spin"

# Edit with custom settings
geminipocket-cli edit-video image.jpg "animate with flowing water" \
  --aspect-ratio 16:9 \
  --resolution 720p \
  --negative-prompt "static, boring"
```

### Configuration

```bash
# Set default output directory
geminipocket-cli config set output_dir ~/Pictures/AI

# Set custom API endpoint
geminipocket-cli config set api_url https://your-custom-endpoint.com

# Get configuration value
geminipocket-cli config get output_dir

# List all configuration
geminipocket-cli config list

# Reset to defaults
geminipocket-cli config reset
```

## 🔌 API Endpoints

### Documentation
- **Interactive API Docs**: [https://geminipocket.guitaripod.workers.dev/docs](https://geminipocket.guitaripod.workers.dev/docs) (Swagger UI)
- **OpenAPI Specification**: [https://geminipocket.guitaripod.workers.dev/openapi](https://geminipocket.guitaripod.workers.dev/openapi)
- **ReDoc Documentation**: [https://geminipocket.guitaripod.workers.dev/redoc](https://geminipocket.guitaripod.workers.dev/redoc)

### Key Endpoints

#### Authentication
- `POST /auth/register` - Register new user
- `POST /auth/login` - User login
- `POST /auth/logout` - User logout
- `GET /auth/status` - Check auth status

#### Image Operations
- `POST /generate` - Generate image from text
- `POST /edit` - Edit existing image

#### Video Operations
- `POST /generate_video` - Generate video from text
- `POST /edit_video` - Edit image into video
- `GET /video_status/{operation_id}` - Check video generation status

#### Utility
- `GET /health` - API health check
- `GET /info` - API information and version

### Authentication
All API requests require authentication via Bearer token:
```
Authorization: Bearer your_api_key_here
```

### Rate Limits
- Image generation: 10 requests/minute
- Video generation: 5 requests/minute
- Status checks: 30 requests/minute

### Supported Formats

#### Images
- **Input**: PNG, JPG, JPEG, GIF, WebP
- **Output**: PNG (1024x1024 resolution)
- **Editing**: Transform, enhance, modify existing images

#### Videos
- **Input**: Image files (same as above for video editing)
- **Output**: MP4 format
- **Duration**: 8 seconds
- **Resolutions**: 720p, 1080p
- **Aspect Ratios**: 16:9, 9:16

### Limitations
- Video generation requires Google AI API access
- Large images may be resized for processing
- Complex prompts may take longer to generate
- API quotas apply based on your Google Cloud plan

## 🛠️ Development

### Prerequisites
- Rust 1.70+ with WebAssembly target
- Node.js 16+ and npm
- Cloudflare Wrangler CLI
- Git

### Setup

```bash
# Clone the repository
git clone https://github.com/yourusername/geminipocket.git
cd geminipocket

# Install Rust WebAssembly target
rustup target add wasm32-unknown-unknown

# Install Cloudflare Wrangler
npm install -g wrangler
```

### Backend Development (Cloudflare Worker)

```bash
# Install dependencies
cd backend/worker
cargo build

# Run locally with live reload
cd backend
wrangler dev

# Build for production
cd backend/worker
cargo build --release

# Run tests
cargo test

# Deploy to production
cd ../..
./build_deploy.sh
```

### CLI Development

```bash
# Build all components
cargo build --workspace

# Build release version
cargo build --workspace --release

# Build CLI only
cargo build -p geminipocket

# Run CLI locally
cargo run --bin geminipocket-cli -- --help

# Install CLI globally
cargo install --path cli
```

### Web Interface Development

```bash
# Install dependencies
cd web
npm install

# Start development server
npm run dev

# Build for production
npm run build
```

### Database Setup

The backend uses Cloudflare D1 for data persistence:

```bash
# Create D1 database
wrangler d1 create geminipocket

# Push migrations
wrangler d1 migrations apply --local
```

### Testing

```bash
# Run all tests
cargo test --workspace

# Run backend tests only
cd backend/worker && cargo test

# Run CLI tests only
cargo test -p geminipocket

# Run with verbose output
cargo test -- --nocapture
```

### Environment Variables

Create a `.env` file in the backend directory:

```env
# Google AI API Key (required for AI features)
GEMINI_API_KEY=your_google_ai_api_key_here

# Database configuration
DATABASE_URL=your_d1_database_url
```

## 🔧 Troubleshooting

### Common Issues

**"API key not configured" error**
- Make sure you've set up your Google AI API key
- For CLI: Run `geminipocket-cli auth login` or set the API key in config
- For API: Include the API key in the Authorization header

**Video generation fails**
- Check your Google AI API quota and billing
- Ensure the API key has access to the Generative Language API
- Try with simpler prompts first

**Slow generation times**
- Video generation can take 1-3 minutes
- Image generation typically takes 10-30 seconds
- Check your internet connection

**Web interface not loading**
- Try clearing your browser cache
- Check if the service is online at the health endpoint
- Ensure JavaScript is enabled

### Getting Help

- **API Documentation**: Visit the `/docs` endpoint for interactive API docs
- **Health Check**: Use `geminipocket-cli health` or visit `/health`
- **Logs**: Check browser console for frontend errors
- **Issues**: Report bugs on GitHub

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Workflow

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/your-feature`
3. Make your changes and add tests
4. Run the test suite: `cargo test --workspace`
5. Commit your changes: `git commit -m "Add your feature"`
6. Push to your fork: `git push origin feature/your-feature`
7. Create a Pull Request

### Code Style

- Follow Rust standard formatting: `cargo fmt`
- Run clippy for linting: `cargo clippy`
- Write tests for new features
- Update documentation for API changes

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Powered by [Google Gemini](https://ai.google.dev/) and [Veo](https://ai.google.dev/veo)
- Built with [Cloudflare Workers](https://workers.cloudflare.com/)
- CLI built with [Clap](https://github.com/clap-rs/clap)
- Web interface uses vanilla JavaScript for maximum compatibility
