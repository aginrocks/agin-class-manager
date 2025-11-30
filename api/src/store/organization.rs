use color_eyre::eyre::Context;
use futures::stream::TryStreamExt;
use mongodb::{
    Collection,
    bson::{doc, oid::ObjectId},
};

use crate::{
    axum_error::AxumResult,
    models::{
        organization::{Organization, PartialOrganization},
        user::OrganizationRole,
    },
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

    pub async fn get_all(&self, user_id: ObjectId) -> AxumResult<Vec<Organization>> {
        let cursor = self
            .collection
            .find(doc! {"members.user_id": user_id })
            .await
            .wrap_err("Failed to fetch courses")?;

        let organizations = cursor
            .try_collect()
            .await
            .wrap_err("Failed to collect courses")?;

        Ok(organizations)
    }

    pub async fn get_by_id(&self, id: ObjectId) -> AxumResult<Option<Organization>> {
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

    pub async fn add_member(
        &self,
        id: ObjectId,
        user_id: ObjectId,
        role: OrganizationRole,
    ) -> AxumResult<()> {
        self.collection
            .find_one_and_update(
                doc! { "_id": id },
                doc! { "$push": { "members": { "user_id": user_id, "role": role } } },
            )
            .await
            .wrap_err("Failed to add member to organization")?;

        Ok(())
    }

    pub async fn delete(&self, id: ObjectId) -> AxumResult<()> {
        self.collection
            .delete_one(doc! {"_id": id})
            .await
            .wrap_err("Failed to delete organization")?;

        Ok(())
    }
}
