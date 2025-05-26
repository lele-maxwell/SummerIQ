pub mod auth;
pub mod upload;
pub mod chat;

pub use auth::auth_router;
pub use upload::upload_router;
pub use chat::chat_router;
