use tracing::{info, Level};

#[tokio::main]
async fn main() {
    // 로깅 설정
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    info!("Zeta4g Graph Database Starting...");
}