use crate::database_object;
use crate::mongo_id::object_id_as_string_required;

use chrono::Utc;
use mongodb::bson::oid::ObjectId;
use partial_struct::Partial;
use serde::{Deserialize, Serialize};
use utoipa::{ToSchema, schema};
use validator::Validate;
use visible::StructFields;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct Payer {
    pub user_id: i64,
    pub paid_amount: i64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct PayerVec(pub Vec<Payer>);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[StructFields(pub)]
pub struct FundraisingRes {
    id: i64,
    name: String,
    description: String,
    payers: Vec<Payer>,
    total_amount: i64,
    start_date: Option<chrono::DateTime<Utc>>,
    end_date: Option<chrono::DateTime<Utc>>,
    organization_id: i64,
}

/// MutableFundraising is used for creating or updating fundraising throught the API.
#[derive(Serialize, Deserialize, ToSchema, Validate)]
pub struct MutableFundraising {
    #[validate(length(min = 1, max = 64))]
    pub name: String,

    #[validate(length(max = 2048))]
    pub description: String,

    pub payers: Vec<Payer>,
    pub total_amount: i64,
    pub start_date: Option<chrono::DateTime<Utc>>,
    pub end_date: Option<chrono::DateTime<Utc>>,
}

use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "fundrisings")]
#[StructFields(pub)]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,

    name: String,

    description: String,

    #[sea_orm(has_many, via = "payers")]
    payers: HasMany<super::user::Entity>,

    total_amount: i64,

    start_date: Option<chrono::DateTime<Utc>>,

    end_date: Option<chrono::DateTime<Utc>>,

    #[sea_orm(indexed)]
    organization_id: i64,
    #[sea_orm(belongs_to, from = "organization_id", to = "id")]
    org: HasOne<super::organization::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
