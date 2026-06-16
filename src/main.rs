use tokio::net::TcpListener;
use tracing::{Level, info};
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

mod app_state;
mod routers;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env().add_directive(Level::INFO.into()))
        .init();

    let app_state = app_state::init_state();
    let app = routers::init_router().with_state(app_state);
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    info!("server started on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
