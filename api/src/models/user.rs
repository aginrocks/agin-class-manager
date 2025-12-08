use sea_orm::entity::prelude::*;

use sea_orm::ActiveModelBehavior;
use serde::Serialize;

// database_object!(Membership {
//     #[schema(value_type = String)]
//     #[serde(with = "object_id_as_string_required")]
//     user_id: ObjectId,
//     role: OrganizationRole,
// });

// database_object!(User {
//     #[serde(rename = "_id", with = "object_id_as_string_required")]
//     #[schema(value_type = String)]
//     id: ObjectId,
//     subject: String,
//     name: String,
//     email: String,
// });

// #[derive(Serialize, Deserialize, ToSchema, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
// #[serde(rename_all = "lowercase")]
// pub enum OrganizationRole {
//     Member = 0,
//     Admin = 1,
// }

// impl From<OrganizationRole> for mongodb::bson::Bson {
//     fn from(scope: OrganizationRole) -> Self {
//         mongodb::bson::serialize_to_bson(&scope).expect("Failed to convert to BSON")
//     }
// }

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
