# GeminiPocket Web

The web interface for GeminiPocket - AI-powered image generation and editing using Google Gemini.

## Features

- **Text-to-Image Generation**: Create images from text prompts
- **Image Editing**: Upload and edit existing images with AI
- **Responsive Design**: Works on desktop and mobile devices
- **Dark Mode Support**: Automatically respects system theme preferences
- **API Documentation**: Direct access to interactive API docs

## Project Structure

```
web/
├── public/           # Static web assets
│   ├── index.html    # Main HTML file
│   ├── styles.css    # CSS styles with dark mode support
│   └── app.js        # JavaScript functionality
├── src/              # Source files (for future expansion)
├── package.json      # Node.js project configuration
└── README.md         # This file
```

## Development

### Prerequisites

- Node.js (for development server)
- Access to GeminiPocket API backend

### Local Development

1. Install dependencies:
   ```bash
   cd web
   npm install
   ```

2. Start development server:
   ```bash
   npm run dev
   ```

3. Open http://localhost:3000 in your browser

### Building for Production

The web app consists of static files that are served directly by the Cloudflare Worker. No build process is required for production deployment.

```bash
npm run build
```

## API Integration

The web app communicates with the GeminiPocket backend API:

- **Base URL**: Configured to work with the deployed Cloudflare Worker
- **Endpoints**:
  - `POST /generate` - Generate images from text
  - `POST /edit` - Edit existing images
  - `GET /health` - API health check
  - `GET /docs` - Swagger UI documentation
  - `GET /openapi` - OpenAPI specification

## Features Overview

### Image Generation
- Text input with character counter (max 10,000 characters)
- Example prompts for inspiration
- Real-time generation with loading indicators
- Download generated images

### Image Editing
- Drag-and-drop file upload
- Support for PNG, JPG, GIF, WebP formats
- Image preview before editing
- Side-by-side comparison of original and edited images
- Download edited images

### User Experience
- Clean, modern interface
- Responsive design for all screen sizes
- Loading states and error handling
- Keyboard navigation support
- Accessibility features

## Deployment

The web app is served directly by the Cloudflare Worker backend. The static files in the `public/` directory are embedded in the worker and served at the root URL.

## Contributing

1. Make changes to files in the `public/` directory
2. Test locally using `npm run dev`
3. The changes will be automatically included when the worker is rebuilt and deployed

## License

GPL V3 License - see the main project LICENSE file for details.