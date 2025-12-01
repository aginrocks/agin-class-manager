use bson::{doc, oid::ObjectId};
use color_eyre::eyre::{self, Context, eyre};
use mongodb::Collection;

use crate::{
    axum_error::{AxumError, AxumResult},
    models::santa::{PartialSanta, Santa, SantaParticipant}, state::AppState,
};

#[derive(Clone)]
pub struct SantaStore {
    pub collection: Collection<Santa>,
    pub partial_collection: Collection<PartialSanta>,
}

pub struct CreateSantaResult {
    pub id: ObjectId,
}

impl SantaStore {
    pub fn new(db: &mongodb::Database) -> Self {
        const COLLECTION: &str = "santa";
        Self {
            collection: db.collection::<Santa>(COLLECTION),
            partial_collection: db.collection::<PartialSanta>(COLLECTION),
        }
    }
    pub async fn create(&self, santa: PartialSanta) -> AxumResult<CreateSantaResult> {
        let res = self
            .partial_collection
            .insert_one(santa)
            .await
            .wrap_err("Failed to create santa")?;

        Ok(CreateSantaResult {
            id: res
                .inserted_id
                .as_object_id()
                .expect("Failed to get inserted id"),
        })
    }

    pub async fn get_by_organization(&self, organization_id: &ObjectId) -> AxumResult<Santa> {
        let santa = self
            .collection
            .find_one(doc! { "organization_id": organization_id })
            .await
            .wrap_err("Failed to fetch Santa")?;

        if santa.is_none() {
            return Err(AxumError::not_found(eyre!(
                "There is no secret santa attached to this organization"
            )));
        }

        Ok(santa.unwrap())
    }
    
   
}
