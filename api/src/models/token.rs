use sea_orm::entity::prelude::*;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

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
