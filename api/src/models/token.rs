use sea_orm::entity::prelude::*;

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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct ScopeVec(pub Vec<Scope>);

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "tokens")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,

    pub user_id: i32,
    #[sea_orm(belongs_to, from = "user_id", to = "id")]
    pub user: HasOne<super::user::Entity>,

    #[sea_orm(unique)]
    pub hashed_token: String,

    pub display_name: String,

    pub scopes: ScopeVec,
}

impl ActiveModelBehavior for ActiveModel {}
