use axum::{
    routing::{get, post, put},
    Router,
};
use config::Config;
use sqlx::{migrate::Migrator, sqlite::SqlitePool};

use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::info;

mod config;
mod error;
mod libs;
mod routes;

static MIGRATOR: Migrator = sqlx::migrate!();

#[derive(Clone)]
pub struct AppState {
    pool: Box<SqlitePool>,
    config: Config,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(
            #[cfg(debug_assertions)]
            tracing::Level::DEBUG,
            #[cfg(not(debug_assertions))]
            tracing::Level::INFO,
        )
        .init();

    let config = Config::new();

    let pool = SqlitePool::connect(&config.database_url).await?;

    MIGRATOR.run(&pool).await?;

    let port = config.port;

    let state = AppState {
        pool: Box::new(pool),
        config,
    };

    let app = Router::new()
        .route("/trips", post(routes::create_trip))
        .route("/trips/:trip_id/confirm", get(routes::confirm_trip))
        .route(
            "/participants/:participant_id/confirm",
            get(routes::confirm_participant),
        )
        .route(
            "/participants/:participant_id",
            get(routes::get_participant),
        )
        .route(
            "/trips/:trip_id/activities",
            post(routes::create_activity).get(routes::get_activities),
        )
        .route(
            "/trips/:trip_id/links",
            post(routes::create_link).get(routes::get_links),
        )
        .route(
            "/trips/:trip_id/participants",
            get(routes::get_participants),
        )
        .route("/trips/:trip_id/invites", post(routes::create_invite))
        .route(
            "/trips/:trip_id",
            put(routes::update_trip).get(routes::get_trip_details),
        )
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addrs = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(&addrs).await?;
    info!("Listening on: {}", addrs);
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
