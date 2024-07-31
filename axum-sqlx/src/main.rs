use axum::{
    routing::{get, post, put},
    Router,
};
use config::Config;
use crossbeam::channel::{unbounded, Receiver, Sender};
use sqlx::{
    migrate::{MigrateDatabase, Migrator},
    // sqlite::SqlitePool,
    postgres::PgPool,
};
use tasks::Task;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::info;

mod config;
mod error;
mod libs;
mod routes;
mod tasks;

static MIGRATOR: Migrator = sqlx::migrate!();

#[derive(Clone)]
pub struct AppState {
    pool: Box<PgPool>,
    config: Config,
    tasks_sender: Sender<Box<dyn Task + Send>>,
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

    // if !sqlx::Sqlite::database_exists(&config.database_url).await? {
    //     sqlx::Sqlite::create_database(&config.database_url).await?;
    // }

    let pool = PgPool::connect(&config.database_url).await?;

    MIGRATOR.run(&pool).await?;

    let port = config.port;

    let (sender, receiver) = unbounded::<Box<dyn Task + Send>>();

    let state = AppState {
        pool: Box::new(pool),
        config,
        tasks_sender: sender.clone(),
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
        .route("/", get(routes::ready))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(state);

    let tasks = tokio::spawn(async move { task_executor(receiver).await });

    let addrs = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(&addrs).await?;
    info!("Listening on: {}", addrs);
    axum::serve(listener, app.into_make_service()).await?;

    drop(sender);

    tasks.await??;

    Ok(())
}

async fn task_executor(receiver: Receiver<Box<dyn Task + Send>>) -> anyhow::Result<()> {
    while let Ok(task) = receiver.recv() {
        task.execute().await?;
    }

    Ok(())
}
