use mongodb::Database;

use crate::store::{fundraising::FundraisingStore, organization::OrganizationStore};

mod fundraising;
mod organization;

#[derive(Clone)]
pub struct DatabaseStore {
    pub organization: OrganizationStore,
    pub fundraising: FundraisingStore,
}

impl DatabaseStore {
    pub fn new(database: &Database) -> Self {
        Self {
            organization: OrganizationStore::new(database),
            fundraising: FundraisingStore::new(database),
        }
    }
}
