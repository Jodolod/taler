use axum::Router;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), SetupError> {
    tracing_subscriber::fmt::init();

    let app = Router::new();

    let addr = std::env::var("LISTEN_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".to_string());
    let listener = TcpListener::bind(&addr).await?;
    tracing::info!("listening on {addr}");
    axum::serve(listener, app).await?;
    Ok(())
}

#[derive(thiserror::Error)]
enum SetupError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

impl std::fmt::Debug for SetupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}
