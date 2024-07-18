use axum::{
    routing::{get, post},
    Router,
};
use sqlx::{migrate::Migrator, sqlite::SqlitePool};
use std::net::SocketAddr;

use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::info;

mod error;
mod libs;
mod routes;

static MIGRATOR: Migrator = sqlx::migrate!();

#[derive(Clone)]
pub struct AppState {
    pool: Box<SqlitePool>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_max_level(
            #[cfg(debug_assertions)]
            tracing::Level::DEBUG,
            #[cfg(not(debug_assertions))]
            tracing::Level::INFO,
        )
        .init();

    let db_url = std::env::var("DATABASE_URL")?;
    let pool = SqlitePool::connect(&db_url).await?;

    MIGRATOR.run(&pool).await?;

    let state = AppState {
        pool: Box::new(pool),
    };

    let app = Router::new()
        .route("/trips", post(routes::create_trip))
        .route("/trips/:trip_id/confirm", get(routes::confirm_trip))
        .route(
            "/participants/:participant_id/confirm",
            get(routes::confirm_participant),
        )
        .route(
            "/trips/:trip_id/activities",
            post(routes::create_activity).get(routes::get_activities),
        )
        .route(
            "/trips/:trip_id/links",
            post(routes::create_link).get(routes::get_links),
        )
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3333));
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    info!("Listening on: {}", addr);
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
