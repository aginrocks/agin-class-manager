use sea_orm::entity::prelude::*;

use sea_orm::ActiveModelBehavior;
use serde::Serialize;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,

    #[sea_orm(unique)]
    pub subject: String,

    pub email: String,

    pub name: String,

    #[sea_orm(has_many, via = "org_members")]
    pub organizations: HasMany<super::organization::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
