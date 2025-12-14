use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use visible::StructFields;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[StructFields(pub)]
pub struct Payer {
    user_id: i64,
    /// If empty, defaults to zero
    paid_amount: Option<i64>,
    /// Custom amout that this user has to pay, if empty, then it's automatically divided evenly between users
    amount_to_pay: Option<i64>,
    comment: String,
}

#[derive(Debug, Clone, ToSchema, Serialize)]
#[StructFields(pub)]
pub struct PayerRes {
    user_id: i64,
    paid_amount: i64,
    amount_to_pay: i64,
    comment: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct PayerVec(pub Vec<Payer>);

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize)]
#[StructFields(pub)]
#[sea_orm(table_name = "payer")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, unique_key = "payer")]
    fundraising_id: i64,
    #[sea_orm(primary_key, auto_increment = false, unique_key = "payer")]
    user_id: i64,
    #[sea_orm(belongs_to, from = "fundraising_id", to = "id")]
    fundraising: Option<super::fundraising::Entity>,
    #[sea_orm(belongs_to, from = "user_id", to = "id")]
    user: Option<super::user::Entity>,
    paid_amount: i64,
    amount_to_pay: i64,
    comment: String,
}

impl ActiveModelBehavior for ActiveModel {}
