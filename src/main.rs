use std::env;

use dotenvy::from_filename;

mod memory;
mod replication;
mod socket;

#[tokio::main]
async fn main() {
    {
        // Dotenv load
        from_filename(".env.dev").ok();
    }
}
