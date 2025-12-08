use axum::{Extension, Json};
use color_eyre::eyre::eyre;
use mongodb::bson::oid::ObjectId;
use sea_orm::{ActiveValue::Set, EntityTrait, ModelTrait};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};
use validator::Validate;

use crate::{
    axum_error::{AxumError, AxumResult},
    middlewares::require_auth::UnauthorizedError,
    models::{
        org_members::{self, OrganizationRole},
        organization, user,
    },
    mongo_id::object_id_as_string_required,
    routes::api::{GenericError, NotFoundError},
    state::AppState,
};

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(add_member_to_organization))
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug, Validate)]
pub struct PatchMemberPayload {
    pub user_id: i64,
    pub role: OrganizationRole,
}

/// Add member to organization
#[utoipa::path(
    method(post),
    path = "/",
    params(
        ("org_id" = String, Path, description = "Organization id"),
    ),
    responses(
        (status = OK, description = "Success", body = String, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json"),
        (status = NOT_FOUND, description = "Organization not found", body = NotFoundError, content_type = "application/json"),
        (status = FORBIDDEN, description = "Forbidden", body = GenericError, content_type = "application/json")
    ),
    request_body = PatchMemberPayload,
    tag = "Organizations Members"
)]
async fn add_member_to_organization(
    Extension(state): Extension<AppState>,
    Extension(org_data): Extension<organization::Model>,
    // Path(org_id): Path<ObjectId>,
    Json(payload): Json<PatchMemberPayload>,
) -> AxumResult<&'static str> {
    if org_data
        .find_related(user::Entity)
        .all(&state.sea_orm)
        .await?
        .iter()
        .any(|m| m.id == payload.user_id)
    {
        return Err(AxumError::bad_request(eyre!(
            "User is already a member of this organization"
        )));
    }

    let members = org_members::ActiveModel {
        user_id: Set(payload.user_id),
        org_id: Set(org_data.id.clone()),
        role: Set(payload.role),
    };

    org_members::Entity::insert(members)
        .exec_without_returning(&state.sea_orm)
        .await?;

    Ok("Ok")
}
