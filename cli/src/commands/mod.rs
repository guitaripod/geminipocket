pub mod auth;
pub mod config;
pub mod edit;
pub mod edit_video;
pub mod generate;
pub mod generate_video;
pub mod health;
pub mod info;

pub use auth::{handle_login, handle_logout, handle_register, handle_status};
pub use self::config::handle_config;
pub use edit::handle_edit;
pub use edit_video::handle_edit_video;
pub use generate::handle_generate;
pub use generate_video::handle_generate_video;
pub use health::handle_health;
pub use info::handle_info;