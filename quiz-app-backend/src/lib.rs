pub mod auth;
pub mod config;
pub mod db;
pub mod error;
pub mod handlers;
pub mod models;
pub mod routes;
pub mod seeder;

pub mod test_helpers;

#[cfg(test)]
mod tests {
    // Internal test modules can go here
}
