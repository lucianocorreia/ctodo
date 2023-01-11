mod config;
use config::config::Config;

#[tokio::main]
async fn main() {
    let config = Config::new();
    println!("Config: {:#?}", config);
}
