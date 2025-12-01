use mongodb::Database;

use crate::store::{
    fundraising::FundraisingStore, organization::OrganizationStore, santa::SantaStore,
    santa_participant::SantaParticipantStore,
};

mod fundraising;
mod organization;
mod santa;
mod santa_participant;

#[derive(Clone)]
pub struct DatabaseStore {
    pub organization: OrganizationStore,
    pub fundraising: FundraisingStore,
    pub santa: SantaStore,
    pub santa_participant: SantaParticipantStore,
}

impl DatabaseStore {
    pub fn new(database: &Database) -> Self {
        Self {
            organization: OrganizationStore::new(database),
            fundraising: FundraisingStore::new(database),
            santa: SantaStore::new(database),
            santa_participant: SantaParticipantStore::new(database),
        }
    }
}
