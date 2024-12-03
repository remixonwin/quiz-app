pub mod auth;
pub mod config;
pub mod db;
pub mod error;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod seeder;
pub mod utils;

pub mod test_helpers;

#[cfg(test)]
mod tests {
    // Internal test modules can go here
}
