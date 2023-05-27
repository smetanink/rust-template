use dotenvy::dotenv;
use env_logger::init;
use log::{info};


fn main() {
  dotenv().unwrap();
  init();
  info!("Initialized")
}
