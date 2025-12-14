use std::sync::Arc;

use crate::settings::Settings;

#[derive(Clone)]
pub struct AppState {
    pub settings: Arc<Settings>,
    pub fred: fred::prelude::Pool,
    pub sea_orm: sea_orm::DatabaseConnection,
}
