use axum::Router;
use sqlx::SqlitePool;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqliteSynchronous};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), SetupError> {
    tracing_subscriber::fmt::init();

    let db_path = std::env::var("DB_PATH").unwrap_or_else(|_| "taler.db".to_string());
    let db_options = SqliteConnectOptions::new()
        .filename(&db_path)
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal);
    let db = SqlitePool::connect_with(db_options).await?;
    tracing::info!("using database at {db_path}");
    sqlx::migrate!().run(&db).await?;

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
    #[error("db connect error: {0}")]
    DbConnect(#[from] sqlx::Error),
    #[error("db migrations error: {0}")]
    DbMigration(#[from] sqlx::migrate::MigrateError),
}

impl std::fmt::Debug for SetupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}
