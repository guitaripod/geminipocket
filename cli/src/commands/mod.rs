pub mod auth;
pub mod config;
pub mod edit;
pub mod generate;
pub mod health;
pub mod info;

pub use auth::{handle_login, handle_logout, handle_register, handle_status};
pub use self::config::handle_config;
pub use edit::handle_edit;
pub use generate::handle_generate;
pub use health::handle_health;
pub use info::handle_info;