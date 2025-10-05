use axum::{Extension, Json, body::Body, extract::Path};
use color_eyre::eyre::{self, eyre};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};
use validator::Validate;

use crate::{
    axum_error::{AxumError, AxumResult},
    middlewares::{require_auth::UnauthorizedError, require_org_permissions::OrganizationData},
    models::user::OrganizationRole,
    mongo_id::object_id_as_string_required,
    routes::api::{GenericError, NotFoundError},
    state::AppState,
};

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(add_member_to_organization))
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug, Validate)]
pub struct PatchMemberPayload {
    #[schema(value_type = String)]
    #[serde(with = "object_id_as_string_required")]
    pub user_id: ObjectId,
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
    Extension(org_data): Extension<OrganizationData>,
    // Path(org_id): Path<ObjectId>,
    Json(payload): Json<PatchMemberPayload>,
) -> AxumResult<&'static str> {
    if org_data
        .0
        .members
        .iter()
        .any(|m| m.user_id == payload.user_id)
    {
        return Err(AxumError::bad_request(eyre!(
            "User is already a member of this organization"
        )));
    }

    state
        .store
        .organization
        .add_member(org_data.0.id, payload.user_id, payload.role)
        .await?;

    Ok("Ok")
}
