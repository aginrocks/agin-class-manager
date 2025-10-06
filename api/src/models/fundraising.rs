use crate::database_object;
use crate::mongo_id::object_id_as_string_required;

use chrono::Utc;
use mongodb::bson::oid::ObjectId;
use partial_struct::Partial;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;
use visible::StructFields;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Payer {
    #[schema(value_type = String)]
    #[serde(with = "object_id_as_string_required")]
    pub user_id: ObjectId,
    pub paid_amount: u64,
}

database_object!(Fundraising {
    #[serde(
        rename = "_id",
        with = "object_id_as_string_required"
    )]
    #[schema(value_type = Option<String>)]
    id: ObjectId,
    name: String,
    description: String,
    payers: Vec<Payer>,
    total_amount: u64,
    start_date: Option<mongodb::bson::DateTime>,
    end_date: Option<mongodb::bson::DateTime>,
    #[schema(value_type = String)]
    #[serde(with = "object_id_as_string_required")]
    organization_id: ObjectId,
});

/// MutableFundraising is used for creating or updating fundraising throught the API.
#[derive(Serialize, Deserialize, ToSchema, Validate)]
pub struct MutableFundraising {
    #[validate(length(min = 1, max = 64))]
    pub name: String,

    #[validate(length(max = 2048))]
    pub description: String,

    pub payers: Vec<Payer>,
    pub total_amount: u64,
    pub start_date: Option<chrono::DateTime<Utc>>,
    pub end_date: Option<chrono::DateTime<Utc>>,
}
