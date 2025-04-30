// src/main.rs

mod config;
mod security;
mod mqtt;
mod blockchain;

#[tokio::main]
async fn main() {
    println!("[SYSTEM] StepMint Backend Validator Starting...");
    mqtt::start_mqtt().await;
}
