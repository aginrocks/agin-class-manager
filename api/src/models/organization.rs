use crate::database_object;
use crate::mongo_id::object_id_as_string_required;

use mongodb::bson::oid::ObjectId;
use partial_struct::Partial;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
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
    avatar_email: Option<String>,
});

database_object!(Membership {
    #[schema(value_type = String)]
    #[serde(with = "object_id_as_string_required")]
    user_id: ObjectId,
    role: OrganizationRole,
});

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum OrganizationRole {
    Member = 0,
    Admin = 1,
}

impl From<OrganizationRole> for mongodb::bson::Bson {
    fn from(scope: OrganizationRole) -> Self {
        mongodb::bson::to_bson(&scope).expect("Failed to convert to BSON")
    }
}
