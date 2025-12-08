use std::sync::Arc;

use mongodb::Database;

use crate::settings::Settings;

#[derive(Clone)]
pub struct AppState {
    // pub database: Database,
    // pub store: DatabaseStore,
    pub settings: Arc<Settings>,
    pub fred: fred::prelude::Pool,
    pub sea_orm: sea_orm::DatabaseConnection,
}
