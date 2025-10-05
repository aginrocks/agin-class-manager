use std::sync::Arc;

use mongodb::Database;

use crate::{settings::Settings, store::DatabaseStore};

#[derive(Clone)]
pub struct AppState {
    pub database: Database,
    pub store: DatabaseStore,
    pub settings: Arc<Settings>,
    pub fred: fred::prelude::Pool,
}
