use sea_orm::entity::prelude::*;
use serde::Serialize;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "payer")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, unique_key = "payer")]
    pub fundraising_id: i64,
    #[sea_orm(primary_key, auto_increment = false, unique_key = "payer")]
    pub user_id: i64,
    #[sea_orm(belongs_to, from = "fundraising_id", to = "id")]
    pub fundraising: Option<super::fundraising::Entity>,
    #[sea_orm(belongs_to, from = "user_id", to = "id")]
    pub user: Option<super::user::Entity>,
    pub paid_amount: i64,
}

impl ActiveModelBehavior for ActiveModel {}
