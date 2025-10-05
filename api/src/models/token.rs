use mongodb::bson::{doc, oid::ObjectId};
use partial_struct::Partial;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use visible::StructFields;

use crate::database_object;
use crate::mongo_id::object_id_as_string_required;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum Permission {
    Read,
    Write,
    Admin,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(tag = "type", content = "slug", rename_all = "lowercase")]
pub enum ScopeType {
    /// Allows access to the user account
    User,
    /// Allows access to all organizations and projects
    Global,
    /// Allows access to an organization
    Org(String),
    // TODO: Implement project access
    // /// Allows access to a project
    // Project(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct Scope {
    pub permission: Permission,
    pub scope: ScopeType,
}

database_object!(AccessToken {
    #[serde(rename = "_id", with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    id: ObjectId,

    #[serde(with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    user_id: ObjectId,

    hashed_token: String,

    display_name: String,

    scopes: Vec<Scope>,
});
