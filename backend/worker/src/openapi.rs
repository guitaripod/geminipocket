use serde_json::json;

pub fn openapi_spec() -> serde_json::Value {
    json!({
        "openapi": "3.1.0",
        "info": {
            "title": "Gemini Worker API",
            "description": "A Cloudflare Worker that provides AI-powered image generation and editing capabilities using Google's Gemini API",
            "version": "0.1.0",
            "contact": {
                "name": "API Support"
            },
            "license": {
                "name": "MIT",
                "url": "https://opensource.org/licenses/MIT"
            }
        },
        "servers": [
            {
                "url": "https://gemini-worker.guitaripod.workers.dev",
                "description": "Production server"
            },
            {
                "url": "http://localhost:8787",
                "description": "Local development server"
            }
        ],
        "paths": {
            "/": {
                "get": {
                    "summary": "API Documentation",
                    "description": "Returns the Swagger UI for interactive API documentation",
                    "operationId": "getApiDocs",
                    "tags": ["System"],
                    "responses": {
                        "200": {
                            "description": "Swagger UI documentation page",
                            "content": {
                                "text/html": {
                                    "schema": {
                                        "type": "string",
                                        "description": "HTML page with Swagger UI"
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "/health": {
                "get": {
                    "summary": "Health Check",
                    "description": "Returns the health status of the API",
                    "operationId": "getHealth",
                    "tags": ["System"],
                    "responses": {
                        "200": {
                            "description": "Service is healthy",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "$ref": "#/components/schemas/HealthStatus"
                                    },
                                    "example": {
                                        "status": "healthy",
                                        "timestamp": 1704067200
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "/openapi": {
                "get": {
                    "summary": "OpenAPI Specification",
                    "description": "Returns the OpenAPI specification for this API",
                    "operationId": "getOpenApiSpec",
                    "tags": ["System"],
                    "responses": {
                        "200": {
                            "description": "OpenAPI specification retrieved successfully",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "object",
                                        "description": "OpenAPI 3.1.0 specification document"
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "/generate": {
                "post": {
                    "summary": "Generate Image from Text",
                    "description": "Generates a new image based on a text prompt using Google's Gemini API",
                    "operationId": "generateImage",
                    "tags": ["Image Generation"],
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": {
                                    "$ref": "#/components/schemas/GenerateImageRequest"
                                },
                                "examples": {
                                    "simple": {
                                        "summary": "Simple generation request",
                                        "value": {
                                            "prompt": "A sunset over mountains with a lake in the foreground"
                                        }
                                    },
                                    "detailed": {
                                        "summary": "Detailed generation request",
                                        "value": {
                                            "prompt": "A futuristic robot standing in a modern office, photorealistic, 4k quality, dramatic lighting"
                                        }
                                    }
                                }
                            }
                        }
                    },
                    "responses": {
                        "200": {
                            "description": "Image generated successfully",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "$ref": "#/components/schemas/ImageResponse"
                                    },
                                    "example": {
                                        "success": true,
                                        "image": "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNkYPhfDwAChwGA60e6kgAAAABJRU5ErkJggg==",
                                        "mime_type": "image/png"
                                    }
                                }
                            }
                        },
                        "400": {
                            "description": "Invalid request body",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "$ref": "#/components/schemas/ErrorResponse"
                                    }
                                }
                            }
                        },
                        "500": {
                            "description": "Internal server error or Gemini API error",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "$ref": "#/components/schemas/ErrorResponse"
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "/register": {
                "post": {
                    "summary": "Register New User",
                    "description": "Creates a new user account and returns an API key",
                    "operationId": "registerUser",
                    "tags": ["Authentication"],
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": {
                                    "$ref": "#/components/schemas/RegisterRequest"
                                },
                                "example": {
                                    "email": "user@example.com",
                                    "password": "securepassword123"
                                }
                            }
                        }
                    },
                    "responses": {
                        "200": {
                            "description": "User registered successfully",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "$ref": "#/components/schemas/AuthResponse"
                                    },
                                    "example": {
                                        "success": true,
                                        "api_key": "gp_1234567890_abcdef"
                                    }
                                }
                            }
                        },
                        "400": {
                            "description": "Invalid request or user already exists",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "$ref": "#/components/schemas/ErrorResponse"
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "/login": {
                "post": {
                    "summary": "User Login",
                    "description": "Authenticates a user and returns their API key",
                    "operationId": "loginUser",
                    "tags": ["Authentication"],
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": {
                                    "$ref": "#/components/schemas/LoginRequest"
                                },
                                "example": {
                                    "email": "user@example.com",
                                    "password": "securepassword123"
                                }
                            }
                        }
                    },
                    "responses": {
                        "200": {
                            "description": "Login successful",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "$ref": "#/components/schemas/AuthResponse"
                                    },
                                    "example": {
                                        "success": true,
                                        "api_key": "gp_1234567890_abcdef"
                                    }
                                }
                            }
                        },
                        "401": {
                            "description": "Invalid credentials",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "$ref": "#/components/schemas/ErrorResponse"
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "/edit": {
                "post": {
                    "summary": "Edit Image with Text Prompt",
                    "description": "Edits an existing image based on a text prompt using Google's Gemini API",
                    "operationId": "editImage",
                    "tags": ["Image Generation"],
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": {
                                    "$ref": "#/components/schemas/EditImageRequest"
                                },
                                "examples": {
                                    "transform": {
                                        "summary": "Transform an image",
                                        "value": {
                                            "prompt": "Transform this blue circle into a yellow sun with rays around it",
                                            "image": "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNkYPhfDwAChwGA60e6kgAAAABJRU5ErkJggg==",
                                            "mime_type": "image/png"
                                        }
                                    },
                                    "enhance": {
                                        "summary": "Enhance an image",
                                        "value": {
                                            "prompt": "Add a dramatic sunset background to this landscape",
                                            "image": "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNkYPhfDwAChwGA60e6kgAAAABJRU5ErkJggg=="
                                        }
                                    }
                                }
                            }
                        }
                    },
                    "responses": {
                        "200": {
                            "description": "Image edited successfully",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "$ref": "#/components/schemas/ImageResponse"
                                    },
                                    "example": {
                                        "success": true,
                                        "image": "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNkYPhfDwAChwGA60e6kgAAAABJRU5ErkJggg==",
                                        "mime_type": "image/png"
                                    }
                                }
                            }
                        },
                        "400": {
                            "description": "Invalid request body",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "$ref": "#/components/schemas/ErrorResponse"
                                    }
                                }
                            }
                        },
                        "500": {
                            "description": "Internal server error or Gemini API error",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "$ref": "#/components/schemas/ErrorResponse"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        },
        "components": {
            "schemas": {
                "ApiInfo": {
                    "type": "object",
                    "required": ["name", "version", "endpoints"],
                    "properties": {
                        "name": {
                            "type": "string",
                            "description": "Name of the API"
                        },
                        "version": {
                            "type": "string",
                            "description": "Version of the API"
                        },
                        "endpoints": {
                            "type": "object",
                            "description": "Available endpoints",
                            "properties": {
                                "generate": {
                                    "type": "string",
                                    "description": "Text-to-image generation endpoint"
                                },
                                "edit": {
                                    "type": "string",
                                    "description": "Image editing endpoint"
                                },
                                "health": {
                                    "type": "string",
                                    "description": "Health check endpoint"
                                }
                            }
                        }
                    }
                },
                "HealthStatus": {
                    "type": "object",
                    "required": ["status", "timestamp"],
                    "properties": {
                        "status": {
                            "type": "string",
                            "enum": ["healthy", "unhealthy"],
                            "description": "Health status of the service"
                        },
                        "timestamp": {
                            "type": "number",
                            "description": "Unix timestamp in milliseconds"
                        }
                    }
                },
                "GenerateImageRequest": {
                    "type": "object",
                    "required": ["prompt"],
                    "properties": {
                        "prompt": {
                            "type": "string",
                            "description": "Text prompt describing the image to generate",
                            "minLength": 1,
                            "maxLength": 10000,
                            "example": "A beautiful sunset over mountains"
                        }
                    }
                },
                "EditImageRequest": {
                    "type": "object",
                    "required": ["prompt", "image"],
                    "properties": {
                        "prompt": {
                            "type": "string",
                            "description": "Text prompt describing how to edit the image",
                            "minLength": 1,
                            "maxLength": 10000,
                            "example": "Add a rainbow to the sky"
                        },
                        "image": {
                            "type": "string",
                            "description": "Base64-encoded image data",
                            "format": "byte",
                            "example": "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNkYPhfDwAChwGA60e6kgAAAABJRU5ErkJggg=="
                        },
                        "mime_type": {
                            "type": "string",
                            "description": "MIME type of the image (defaults to image/jpeg if not specified)",
                            "enum": ["image/jpeg", "image/png", "image/gif", "image/webp"],
                            "default": "image/jpeg",
                            "example": "image/png"
                        }
                    }
                },
                "ImageResponse": {
                    "type": "object",
                    "required": ["success", "image", "mime_type"],
                    "properties": {
                        "success": {
                            "type": "boolean",
                            "description": "Whether the operation was successful"
                        },
                        "image": {
                            "type": "string",
                            "description": "Base64-encoded generated or edited image",
                            "format": "byte"
                        },
                        "mime_type": {
                            "type": "string",
                            "description": "MIME type of the returned image",
                            "example": "image/png"
                        }
                    }
                },
                "ErrorResponse": {
                    "type": "object",
                    "required": ["error"],
                    "properties": {
                        "error": {
                            "type": "string",
                            "description": "Error message describing what went wrong"
                        }
                    }
                },
                "RegisterRequest": {
                    "type": "object",
                    "required": ["email", "password"],
                    "properties": {
                        "email": {
                            "type": "string",
                            "format": "email",
                            "description": "User's email address",
                            "example": "user@example.com"
                        },
                        "password": {
                            "type": "string",
                            "description": "User's password",
                            "minLength": 6,
                            "example": "securepassword123"
                        }
                    }
                },
                "LoginRequest": {
                    "type": "object",
                    "required": ["email", "password"],
                    "properties": {
                        "email": {
                            "type": "string",
                            "format": "email",
                            "description": "User's email address",
                            "example": "user@example.com"
                        },
                        "password": {
                            "type": "string",
                            "description": "User's password",
                            "example": "securepassword123"
                        }
                    }
                },
                "AuthResponse": {
                    "type": "object",
                    "required": ["success"],
                    "properties": {
                        "success": {
                            "type": "boolean",
                            "description": "Whether the authentication was successful"
                        },
                        "api_key": {
                            "type": "string",
                            "description": "API key for authenticated requests",
                            "example": "gp_1234567890_abcdef"
                        },
                        "error": {
                            "type": "string",
                            "description": "Error message if authentication failed"
                        }
                    }
                }
            }
        },
        "tags": [
            {
                "name": "Authentication",
                "description": "User authentication and registration endpoints"
            },
            {
                "name": "System",
                "description": "System and health endpoints"
            },
            {
                "name": "Image Generation",
                "description": "Endpoints for generating and editing images using Gemini AI"
            }
        ]
    })
}