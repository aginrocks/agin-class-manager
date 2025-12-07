use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use visible::StructFields;

#[derive(
    Serialize,
    Deserialize,
    ToSchema,
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    FromJsonQueryResult,
)]
#[serde(rename_all = "lowercase")]
pub enum OrganizationRole {
    Member = 0,
    Admin = 1,
}

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "org_members")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub org_id: i64,
    #[sea_orm(primary_key, auto_increment = false)]
    pub user_id: i64,
    #[sea_orm(belongs_to, from = "org_id", to = "id")]
    pub organization: Option<super::organization::Entity>,
    #[sea_orm(belongs_to, from = "user_id", to = "id")]
    pub user: Option<super::user::Entity>,
    pub role: OrganizationRole,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Serialize, Deserialize, ToSchema)]
#[StructFields(pub)]
pub struct Membership {
    user_id: i64,
    role: OrganizationRole,
}
