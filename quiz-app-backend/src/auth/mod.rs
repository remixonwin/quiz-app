pub mod jwt;
pub mod password;
pub mod error;
#[cfg(test)]
mod tests;

pub use error::AuthError;
// Removed unused imports
