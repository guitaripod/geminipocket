use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

use geminipocket::{
    api::GeminiClient,
    commands::{self, config::ConfigAction},
    types::Config,
};

#[derive(Parser)]
#[command(name = "geminipocket-cli")]
#[command(author = "guitaripod")]
#[command(version)]
#[command(about = "Generate and edit AI images with Google Gemini")]
#[command(long_about = "
GeminiPocket - AI Image & Video Generation CLI

Generate stunning images and videos from text prompts or edit existing images
using Google's Gemini AI model. Images are generated at 1024x1024 resolution
in PNG format. Videos are 8-second MP4 files at 720p/1080p resolution.

EXAMPLES:
   Generate an image:
     geminipocket-cli generate \"a sunset over mountains\"

   Generate with custom name:
     geminipocket-cli generate \"abstract art\" --name my-art

   Edit an existing image:
     geminipocket-cli edit photo.png \"add a rainbow\"

   Generate a video:
     geminipocket-cli generate-video \"drone shot following a car along coastal road\"

   Generate video with options:
     geminipocket-cli generate-video \"majestic lion in savannah\" --aspect-ratio 9:16 --resolution 1080p

   Edit image into video:
     geminipocket-cli edit-video photo.png \"make it dance and spin\"

   Configure default output directory:
     geminipocket-cli config set output_dir ~/Videos/AI
")]
struct Cli {
    /// API endpoint URL (can also set GEMINI_API_URL env var)
    #[arg(long, env = "GEMINI_API_URL", default_value = "https://geminipocket.guitaripod.workers.dev", hide_default_value = false)]
    api_url: String,

    /// Default output directory for all generated images
    #[arg(long, short, value_name = "DIR")]
    output: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a new image from text description
    #[command(visible_alias = "gen")]
    Generate {
        /// Describe what you want to generate
        #[arg(value_name = "PROMPT")]
        prompt: String,

        /// Custom filename (timestamp will be added)
        #[arg(short, long, value_name = "NAME")]
        name: Option<String>,

        /// Save to current directory (overrides config)
        #[arg(short, long)]
        save: bool,
    },
    
    /// Transform an existing image using AI
    Edit {
        /// Path to the image file (PNG, JPG, GIF, WebP)
        #[arg(value_name = "IMAGE")]
        image: PathBuf,

        /// Describe how to transform the image
        #[arg(value_name = "PROMPT")]
        prompt: String,

        /// Custom filename (timestamp will be added)
        #[arg(short, long, value_name = "NAME")]
        name: Option<String>,

        /// Save to current directory (overrides config)
        #[arg(short, long)]
        save: bool,
    },

    /// Generate a video from text description
    #[command(visible_alias = "gen-video")]
    GenerateVideo {
        /// Describe what you want to generate
        #[arg(value_name = "PROMPT")]
        prompt: String,

        /// Custom filename (timestamp will be added)
        #[arg(short, long, value_name = "NAME")]
        name: Option<String>,

        /// Save to current directory (overrides config)
        #[arg(short, long)]
        save: bool,

        /// Negative prompt to avoid certain elements
        #[arg(long, value_name = "NEG_PROMPT")]
        negative_prompt: Option<String>,

        /// Aspect ratio (16:9 or 9:16)
        #[arg(long, value_name = "RATIO", default_value = "16:9")]
        aspect_ratio: String,

        /// Resolution (720p or 1080p)
        #[arg(long, value_name = "RES", default_value = "720p")]
        resolution: String,
    },

    /// Transform an existing image into a video using AI
    #[command(visible_alias = "edit-video")]
    EditVideo {
        /// Path to the image file (PNG, JPG, GIF, WebP)
        #[arg(value_name = "IMAGE")]
        image: PathBuf,

        /// Describe how to animate the image
        #[arg(value_name = "PROMPT")]
        prompt: String,

        /// Custom filename (timestamp will be added)
        #[arg(short, long, value_name = "NAME")]
        name: Option<String>,

        /// Save to current directory (overrides config)
        #[arg(short, long)]
        save: bool,

        /// Negative prompt to avoid certain elements
        #[arg(long, value_name = "NEG_PROMPT")]
        negative_prompt: Option<String>,

        /// Aspect ratio (16:9 or 9:16)
        #[arg(long, value_name = "RATIO", default_value = "16:9")]
        aspect_ratio: String,

        /// Resolution (720p or 1080p)
        #[arg(long, value_name = "RES", default_value = "720p")]
        resolution: String,
    },
    
    /// Configure settings (API URL, output directory)
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
    
    /// Check if the API is online and responding
    Health,
    
    /// Show API version and available endpoints
    Info,

    /// Authentication commands (login, register, logout, status)
    Auth {
        #[command(subcommand)]
        action: AuthAction,
    },
}

#[derive(Subcommand)]
enum AuthAction {
    /// Register a new account
    Register,

    /// Login to existing account
    Login,

    /// Logout and remove stored credentials
    Logout,

    /// Show authentication status
    Status,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut config = Config::load()?;

    let api_url = cli.api_url.as_str();
    let output_dir = cli.output.as_deref().or(config.output_dir.as_deref());

    let client = if let Some(api_key) = &config.api_key {
        GeminiClient::with_api_key(api_url.to_string(), api_key.clone())
    } else {
        GeminiClient::new(api_url.to_string())
    };

    match cli.command {
        Commands::Generate { prompt, name, save } => {
            commands::handle_generate(
                &client,
                &prompt,
                output_dir,
                name.as_deref(),
                save,
            )
            .await?;
        }
        Commands::Edit {
            image,
            prompt,
            name,
            save,
        } => {
            commands::handle_edit(
                &client,
                &image,
                &prompt,
                output_dir,
                name.as_deref(),
                save,
            )
            .await?;
        }
        Commands::GenerateVideo {
            prompt,
            name,
            save,
            negative_prompt,
            aspect_ratio,
            resolution,
        } => {
            commands::handle_generate_video(
                &client,
                &prompt,
                output_dir,
                name.as_deref(),
                save,
                negative_prompt.as_deref(),
                Some(&aspect_ratio),
                Some(&resolution),
            )
            .await?;
        }
        Commands::EditVideo {
            image,
            prompt,
            name,
            save,
            negative_prompt,
            aspect_ratio,
            resolution,
        } => {
            commands::handle_edit_video(
                &client,
                &image,
                &prompt,
                output_dir,
                name.as_deref(),
                save,
                negative_prompt.as_deref(),
                Some(&aspect_ratio),
                Some(&resolution),
            )
            .await?;
        }
        Commands::Config { action } => {
            commands::handle_config(action)?;
        }
        Commands::Health => {
            commands::handle_health(&client).await?;
        }
        Commands::Info => {
            commands::handle_info(&client).await?;
        }
        Commands::Auth { action } => {
            match action {
                AuthAction::Register => {
                    commands::handle_register(&client, &mut config).await?;
                }
                AuthAction::Login => {
                    commands::handle_login(&client, &mut config).await?;
                }
                AuthAction::Logout => {
                    commands::handle_logout(&mut config)?;
                }
                AuthAction::Status => {
                    commands::handle_status(&config)?;
                }
            }
        }
    }

    Ok(())
}