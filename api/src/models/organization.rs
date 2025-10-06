use crate::database_object;
use crate::mongo_id::object_id_as_string_required;

use crate::models::user::Membership;
use crate::validators::slug_validator;
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
    #[schema(value_type = Option<String>)]
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
    pub slug: String,

    #[validate(length(max = 500))]
    pub avatar_url: Option<String>,
}
