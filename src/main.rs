mod routes;
mod models;
mod store;

use axum::{Router, routing::{get, post, patch, delete}};
use axum_prometheus::PrometheusMetricLayer;
use routes::{health::health, tasks::*};
use store::Store;
use std::{net::SocketAddr, time::Duration};
use tower_http::{trace::TraceLayer, cors::{Any, CorsLayer}};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(OpenApi)]
#[openapi(
    paths(
        routes::health::health,
        list_tasks, create_task, get_task, patch_task, delete_task
    ),
    components(
        schemas(models::Task, models::NewTask, models::TaskPatch, models::ApiError)
    ),
    tags(
        (name = "tasks", description = "Task management API")
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info,tower_http=info".into()))
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    let store = Store::new("sqlite://data.db").await?;
    store.migrate().await?;

    let (prom_layer, metric_handle) = PrometheusMetricLayer::pair();
    let cors = CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any);

    let api = Router::new()
        .route("/v1/health", get(health))
        .route("/v1/tasks", get(list_tasks).post(create_task))
        .route("/v1/tasks/:id", get(get_task).patch(patch_task).delete(delete_task))
        .with_state(store.clone());

    let app = Router::new()
        .nest("/api", api)
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/metrics", get(|| async move { metric_handle.render() }))
        .layer(cors)
        .layer(prom_layer)
        .layer(TraceLayer::new_for_http());

    let addr: SocketAddr = "0.0.0.0:8080".parse().unwrap();
    tracing::info!(%addr, "starting solstice");
    let server = axum::Server::bind(&addr).serve(app.into_make_service());

    let graceful = server.with_graceful_shutdown(shutdown_signal());
    if let Err(e) = graceful.await {
        tracing::error!(error=?e, "server error");
    }
    Ok(())
}

async fn shutdown_signal() {
    use tokio::signal;
    let ctrl_c = async {
        signal::ctrl_c().await.expect("failed to install Ctrl+C handler");
    };
    #[cfg(unix)]
    let term = async {
        use tokio::signal::unix::{signal, SignalKind};
        let mut sigterm = signal(SignalKind::terminate()).expect("failed to install SIGTERM handler");
        sigterm.recv().await;
    };
    #[cfg(not(unix))]
    let term = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = term => {},
    }
    tracing::info!("shutdown signal received");
}
