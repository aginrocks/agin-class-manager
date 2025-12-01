use crate::models::user::{Membership, User};
use crate::mongo_id::object_id_as_string_required;
use crate::state::AppState;
use crate::validators::slug_validator;
use crate::{database_object, models::user::OrganizationRole};

use bson::doc;
use color_eyre::eyre::Result;
use futures::stream::TryStreamExt;
use mongodb::bson::oid::ObjectId;
use partial_struct::Partial;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;
use visible::StructFields;

database_object!(Organization {
    #[serde(
        rename = "_id",
        with = "object_id_as_string_required"
    )]
    #[schema(value_type = String)]
    id: ObjectId,
    name: String,
    description: String,
    slug: String,
    members: Vec<Membership>,
    avatar_url: Option<String>,
    budget: u64,
});

/// MutableOrganization is used for creating or updating organization throught the API.
#[derive(Serialize, Deserialize, ToSchema, Validate)]
pub struct MutableOrganization {
    #[validate(length(min = 1, max = 32))]
    pub name: String,

    #[validate(length(max = 2048))]
    pub description: String,

    #[validate(custom(function = "slug_validator"), length(min = 1, max = 32))]
    pub slug: Option<String>,

    #[validate(length(max = 500))]
    pub avatar_url: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema, Validate)]
#[StructFields(pub)]
pub struct OrgUser {
    #[schema(value_type = String)]
    #[serde(with = "object_id_as_string_required")]
    id: ObjectId,
    role: OrganizationRole,
    email: String,
    name: String,
}

#[derive(Serialize, Deserialize, ToSchema, Validate)]
#[StructFields(pub)]
pub struct PopulatedOrganization {
    #[serde(rename = "_id", with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    id: ObjectId,
    name: String,
    description: String,
    slug: String,
    members: Vec<OrgUser>,
    avatar_url: Option<String>,
    budget: u64,
}

impl Organization {
    pub async fn populate_users(&self, state: AppState) -> Result<PopulatedOrganization> {
        let users = state.database.collection::<User>("users").find(
            doc! {"_id": {"$in": self.members.iter().map(|g| g.user_id.to_owned()).collect::<Vec<_>>() }},
        ).await?;

        let users: Vec<User> = users.try_collect().await?;

        let members: Vec<OrgUser> = self
            .members
            .iter()
            .filter_map(|member| {
                users
                    .iter()
                    .find(|user| user.id == member.user_id)
                    .map(|user| OrgUser {
                        id: user.id,
                        role: member.role.clone(),
                        email: user.email.clone(),
                        name: user.name.clone(),
                    })
            })
            .collect();

        let res = PopulatedOrganization {
            id: self.id,
            name: self.name.to_owned(),
            description: self.description.to_owned(),
            slug: self.slug.to_owned(),
            members,
            avatar_url: self.avatar_url.to_owned(),
            budget: self.budget,
        };
        Ok(res)
    }
}
