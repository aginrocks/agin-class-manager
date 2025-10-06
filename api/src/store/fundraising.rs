use color_eyre::eyre::Context;
use mongodb::{Collection, bson::oid::ObjectId};

use crate::{
    axum_error::AxumResult,
    models::fundraising::{Fundraising, PartialFundraising},
};

#[derive(Clone)]
pub struct FundraisingStore {
    pub collection: Collection<Fundraising>,
    pub partial_collection: Collection<PartialFundraising>,
}

pub struct CreateFundraisingResult {
    pub id: ObjectId,
}

impl FundraisingStore {
    pub fn new(db: &mongodb::Database) -> Self {
        const COLLECTION: &str = "fundraisings";
        Self {
            collection: db.collection::<Fundraising>(COLLECTION),
            partial_collection: db.collection::<PartialFundraising>(COLLECTION),
        }
    }

    pub async fn create(
        &self,
        fundraising: PartialFundraising,
    ) -> AxumResult<CreateFundraisingResult> {
        let fundraising = self
            .partial_collection
            .insert_one(fundraising)
            .await
            .wrap_err("Failed to create fundraising")?;

        Ok(CreateFundraisingResult {
            id: fundraising
                .inserted_id
                .as_object_id()
                .expect("Failed to get inserted id"),
        })
    }
}
