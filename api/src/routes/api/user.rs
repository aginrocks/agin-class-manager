use crate::{
    axum_error::AxumResult,
    models::{organization, user},
};
use ::serde::Serialize;
use axum::{Extension, Json};
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};
use visible::StructFields;

use crate::{middlewares::require_auth::UnauthorizedError, state::AppState};

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(get_user))
}

#[derive(Serialize, ToSchema)]
#[StructFields(pub)]
pub struct StrippedOrg {
    id: i64,
    name: String,
    avatar_url: Option<String>,
}

impl Into<StrippedOrg> for &organization::Model {
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
    id: i64,
    subject: String,
    name: String,
    email: String,
    organizations: Vec<StrippedOrg>,
}

impl From<user::Model> for GetUserRes {
    fn from(user: user::Model) -> Self {
        Self {
            id: user.id,
            subject: user.subject,
            name: user.name,
            email: user.email,
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
    Extension(user): Extension<user::Model>,
    Extension(organizations): Extension<Vec<organization::Model>>,
) -> AxumResult<Json<GetUserRes>> {
    Ok(Json(GetUserRes {
        organizations: organizations.iter().map(|m| m.into()).collect(),
        ..user.into()
    }))
}
