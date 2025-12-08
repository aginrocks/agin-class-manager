use chrono::Utc;
use color_eyre::eyre::Result;
use sea_orm::ActiveModelBehavior;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;
use visible::StructFields;

use crate::models::santa_participants::SantaParticipant;

#[derive(Serialize, Deserialize, ToSchema, Validate)]
#[StructFields(pub)]
pub struct MutableSanta {
    participants: Vec<i64>,
    start_date: Option<chrono::DateTime<Utc>>,
    propositions_due: Option<chrono::DateTime<Utc>>,
    end_date: Option<chrono::DateTime<Utc>>,
}

#[derive(Serialize, ToSchema, Validate)]
#[StructFields(pub)]
pub struct PopulatedSanta {
    id: i64,
    start_date: chrono::DateTime<Utc>,
    propositions_due: Option<chrono::DateTime<Utc>>,
    end_date: chrono::DateTime<Utc>,
    participants: Vec<SantaParticipant>,
    organization_id: i64,
}

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, ToSchema)]
#[sea_orm(table_name = "secret_santas")]
#[StructFields(pub)]
pub struct Model {
    #[sea_orm(primary_key)]
    id: i64,

    start_date: chrono::DateTime<Utc>,

    propositions_due: Option<chrono::DateTime<Utc>>,

    end_date: chrono::DateTime<Utc>,

    #[sea_orm(has_many, relation_enum = "Participant", via = "santa_participants")]
    #[schema(value_type = ())]
    participants: HasMany<super::user::Entity>,

    #[sea_orm(indexed, unique)]
    organization_id: i64,
    #[sea_orm(belongs_to, from = "organization_id", to = "id")]
    #[schema(value_type = ())]
    org: HasOne<super::organization::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
