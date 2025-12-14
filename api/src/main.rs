mod axum_error;
mod init;
mod middlewares;
pub mod models;
mod routes;
mod settings;
mod state;
pub mod validators;

use std::sync::Arc;

use color_eyre::Result;
use color_eyre::eyre::WrapErr;
use tracing::info;
use utoipa::OpenApi;

use crate::{
    init::{init_axum, init_listener, init_redis, init_sea_orm, init_session_store, init_tracing},
    settings::Settings,
    state::AppState,
};

#[derive(OpenApi)]
#[openapi()]
struct ApiDoc;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    dotenvy::dotenv().ok();

    init_tracing().wrap_err("failed to set global tracing subscriber")?;

    let settings = Arc::new(Settings::try_load()?);

    let fred = init_redis(&settings).await?;

    let sea_orm = init_sea_orm(&settings).await?;

    let app_state = AppState {
        settings: settings.clone(),
        fred: fred.clone(),
        sea_orm,
    };

    let session_layer = init_session_store(&settings, fred).await?;
    let app = init_axum(app_state, session_layer).await?;
    let listener = init_listener(&settings).await?;

    info!(
        "listening on {} ({})",
        listener
            .local_addr()
            .wrap_err("failed to get local address")?,
        settings.general.public_url
    );

    axum::serve(listener, app.into_make_service())
        .await
        .wrap_err("failed to run server")?;

    Ok(())
}
