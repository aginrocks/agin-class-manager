use sea_orm::entity::prelude::*;
use serde::Serialize;
use utoipa::ToSchema;
use visible::StructFields;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, ToSchema)]
#[sea_orm(table_name = "santa_participants")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, unique_key = "santa_participant")]
    pub santa_id: i64,
    #[sea_orm(primary_key, auto_increment = false, unique_key = "santa_participant")]
    pub user_id: i64,
    #[sea_orm(belongs_to, from = "santa_id", to = "id")]
    #[schema(value_type = ())]
    pub santa: Option<super::santa::Entity>,
    #[sea_orm(belongs_to, relation_enum = "Participant", from = "user_id", to = "id")]
    #[schema(value_type = ())]
    pub user: HasOne<super::user::Entity>,

    pub receiver_id: Option<i64>,
    #[sea_orm(
        belongs_to,
        relation_enum = "Receiver",
        from = "receiver_id",
        to = "id"
    )]
    #[schema(value_type = ())]
    pub receiver: Option<super::user::Entity>,

    pub proposition: Option<String>,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Serialize, ToSchema, Debug)]
#[StructFields(pub)]
pub struct SantaParticipant {
    id: i64,
    email: String,
    name: String,
    receiver: Option<i64>,
    proposition: Option<String>,
}
