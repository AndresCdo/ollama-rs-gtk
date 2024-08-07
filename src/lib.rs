pub mod api;
pub mod menu;
pub mod preview;
pub mod ui;
pub mod utils;

pub const OLLAMA_API_URL: &str = "http://localhost:11434/api/generate";
pub const REQUEST_TIMEOUT: u64 = 10000;
pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
