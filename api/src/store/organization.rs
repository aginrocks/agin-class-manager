use color_eyre::eyre::Context;
use futures::stream::TryStreamExt;
use mongodb::{Collection, bson::doc};

use crate::{
    axum_error::AxumResult,
    models::organization::{Organization, PartialOrganization},
};

#[derive(Clone)]
pub struct OrganizationStore {
    pub collection: Collection<Organization>,
    pub partial_collection: Collection<PartialOrganization>,
}

impl OrganizationStore {
    pub fn new(db: &mongodb::Database) -> Self {
        const COLLECTION: &str = "organizations";
        Self {
            collection: db.collection::<Organization>(COLLECTION),
            partial_collection: db.collection::<PartialOrganization>(COLLECTION),
        }
    }

    pub async fn get_all(&self) -> AxumResult<Vec<Organization>> {
        let cursor = self
            .collection
            .find(doc! {})
            .await
            .wrap_err("Failed to fetch courses")?;

        let organizations = cursor
            .try_collect()
            .await
            .wrap_err("Failed to collect courses")?;

        Ok(organizations)
    }

    pub async fn get_by_id(&self, id: &str) -> AxumResult<Option<Organization>> {
        let organization = self
            .collection
            .find_one(doc! { "_id": id })
            .await
            .wrap_err("Failed to fetch organization by id")?;

        Ok(organization)
    }

    pub async fn get_by_slug(&self, slug: &str) -> AxumResult<Option<Organization>> {
        let organization = self
            .collection
            .find_one(doc! { "slug": slug })
            .await
            .wrap_err("Failed to fetch organization by slug")?;

        Ok(organization)
    }
}
