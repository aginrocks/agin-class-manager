use mongodb::Database;

use crate::store::organization::OrganizationStore;

mod organization;

#[derive(Clone)]
pub struct DatabaseStore {
    pub organization: OrganizationStore,
}

impl DatabaseStore {
    pub fn new(database: &Database) -> Self {
        Self {
            organization: OrganizationStore::new(database),
        }
    }
}
