use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Extension, Json, Router,
};
use axum_macros::debug_handler;
use axum_server::{CreateUser, UserRepository};
use kizuna::Locator;
use std::{net::SocketAddr, sync::Arc};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let locator = create_locator().await;

    let app = Router::new()
        .route("/", get(get_users))
        .route("/create", post(create_user))
        .layer(Extension(Arc::new(locator)));

    let addr = SocketAddr::from(([127, 0, 0, 1], 15000));
    tracing::info!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
#[debug_handler]
async fn get_users(Extension(locator): Extension<Arc<Locator>>) -> Response {
    let repo = locator
        .get::<Box<dyn UserRepository + Send + Sync>>()
        .expect("unable to get user repository");

    match repo.get_all().await {
        Ok(users) => Json(users).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

#[debug_handler]
async fn create_user(
    Extension(locator): Extension<Arc<Locator>>,
    Json(payload): Json<CreateUser>,
) -> Response {
    let mut repo = locator
        .get::<Box<dyn UserRepository + Send + Sync>>()
        .expect("unable to get user repository");

    match repo.save(payload).await {
        Ok(user) => Json(user).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

#[allow(unused_mut)]
async fn create_locator() -> Locator {
    let mut locator = Locator::new();

    #[cfg(feature = "postgres")]
    {
        use axum_server::postgres::PostgresUserRepository;
        use sqlx::{pool::PoolOptions, Pool, Postgres};

        let pool = PoolOptions::<Postgres>::new()
            .max_connections(5)
            .connect("postgres://postgres:p455w0rd@localhost:15432/my_database")
            .await
            .unwrap();

        locator.insert(pool);
        locator.insert_with::<_, Box<dyn UserRepository + Send + Sync>>(|locator| {
            let pool = locator
                .get::<Pool<Postgres>>()
                .expect("failed to get in postgres pool");
            Box::new(PostgresUserRepository::new(pool))
        });

        tracing::info!("Using postgres database");
    }

    #[cfg(feature = "memory")]
    {
        use axum_server::memory::{InMemoryUserRepository, MemoryDb};

        let db = MemoryDb::default();
        locator.insert(db);
        locator.insert_with::<_, Box<dyn UserRepository + Send + Sync>>(|locator| {
            let db = locator
                .get::<MemoryDb>()
                .expect("failed to get in memory database");
            Box::new(InMemoryUserRepository::new(db))
        });

        tracing::info!("Using in memory database");
    }

    locator
}
