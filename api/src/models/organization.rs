use crate::models::org_members::{Membership, OrganizationRole};
use crate::models::{organization, user};
use crate::mongo_id::object_id_as_string_required;
use crate::state::AppState;
use crate::validators::slug_validator;

use bson::doc;
use color_eyre::eyre::Result;
use futures::stream::TryStreamExt;
use mongodb::bson::oid::ObjectId;
use partial_struct::Partial;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use utoipa::openapi::schema;
use validator::Validate;
use visible::StructFields;

/// MutableOrganization is used for creating or updating organization throught the API.
#[derive(Serialize, Deserialize, ToSchema, Validate)]
pub struct MutableOrganization {
    #[validate(length(min = 1, max = 32))]
    pub name: String,

    #[validate(length(max = 2048))]
    pub description: String,

    pub slug: Option<String>,

    #[validate(length(max = 500))]
    pub avatar_url: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema, Validate)]
#[StructFields(pub)]
pub struct OrgUser {
    id: i64,
    role: OrganizationRole,
    email: String,
    name: String,
}

#[derive(Serialize, Deserialize, ToSchema, Validate)]
#[StructFields(pub)]
pub struct PopulatedOrganization {
    id: i64,
    name: String,
    description: String,
    slug: String,
    members: Vec<OrgUser>,
    avatar_url: Option<String>,
    budget: i64,
}

#[derive(Serialize, Deserialize, ToSchema, Validate)]
#[StructFields(pub)]
pub struct OrganizationRes {
    id: i64,
    name: String,
    description: String,
    slug: String,
    members: Vec<Membership>,
    avatar_url: Option<String>,
    budget: i64,
}

use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, ToSchema)]
#[sea_orm(table_name = "organizations")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,

    pub name: String,

    pub description: String,

    #[sea_orm(unique)]
    pub slug: String,

    pub avatar_url: Option<String>,

    pub budget: i64,

    #[sea_orm(has_many, via = "org_members")]
    #[schema(value_type = ())]
    pub members: HasMany<super::user::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
