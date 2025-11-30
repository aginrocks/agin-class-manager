use crate::{axum_error::AxumResult, mongo_id::object_id_as_string_required};
use ::serde::{Deserialize, Serialize};
use axum::{Extension, Json};
use bson::oid::ObjectId;
use chrono::serde;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};
use visible::StructFields;

use crate::{
    middlewares::require_auth::{UnauthorizedError, UserData},
    models::{organization::Organization, user::User},
    state::AppState,
};

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(get_user))
}

#[derive(Serialize, ToSchema)]
#[StructFields(pub)]
pub struct StrippedOrg {
    #[serde(rename = "_id", with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    id: ObjectId,
    name: String,
    avatar_url: Option<String>,
}

impl Into<StrippedOrg> for &Organization {
    fn into(self) -> StrippedOrg {
        StrippedOrg {
            id: self.id,
            avatar_url: self.avatar_url.to_owned(),
            name: self.name.to_owned(),
        }
    }
}

#[derive(Serialize, ToSchema, Default)]
#[StructFields(pub)]
pub struct GetUserRes {
    #[serde(rename = "_id", with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    id: ObjectId,
    subject: String,
    name: String,
    email: String,
    organizations: Vec<StrippedOrg>,
}

impl Into<GetUserRes> for User {
    fn into(self) -> GetUserRes {
        GetUserRes {
            id: self.id,
            subject: self.subject,
            name: self.name,
            email: self.email,
            ..Default::default()
        }
    }
}

/// Get user details
#[utoipa::path(
    method(get),
    path = "/",
    responses(
        (status = OK, description = "Success", body = GetUserRes, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json")
    ),
    tag = "Auth"
)]
async fn get_user(
    Extension(user): Extension<UserData>,
    Extension(state): Extension<AppState>,
) -> AxumResult<Json<GetUserRes>> {
    let organizations = state.store.organization.get_all(user.0.id).await?;

    Ok(Json(GetUserRes {
        organizations: organizations.iter().map(|m| m.into()).collect(),
        ..user.0.into()
    }))
}
