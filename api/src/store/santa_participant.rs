use bson::oid::ObjectId;
use color_eyre::eyre::Context;
use mongodb::Collection;

use crate::{
    axum_error::AxumResult,
    models::santa::{PartialSantaParticipant, SantaParticipant},
};

#[derive(Clone)]
pub struct SantaParticipantStore {
    pub collection: Collection<SantaParticipant>,
    pub partial_collection: Collection<PartialSantaParticipant>,
}

pub struct CreateSantaParticipantResult {
    pub id: ObjectId,
}

impl SantaParticipantStore {
    pub fn new(db: &mongodb::Database) -> Self {
        const COLLECTION: &str = "santa-participants";
        Self {
            collection: db.collection::<SantaParticipant>(COLLECTION),
            partial_collection: db.collection::<PartialSantaParticipant>(COLLECTION),
        }
    }
    pub async fn create(
        &self,
        participant: PartialSantaParticipant,
    ) -> AxumResult<CreateSantaParticipantResult> {
        let res = self
            .partial_collection
            .insert_one(participant)
            .await
            .wrap_err("Failed to create santa participant")?;

        Ok(CreateSantaParticipantResult {
            id: res
                .inserted_id
                .as_object_id()
                .expect("Failed to get inserted id"),
        })
    }
}
