use axum::{
    Extension,
    extract::{Path, Request},
    middleware::Next,
    response::Response,
};
use color_eyre::eyre::eyre;
use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    axum_error::{AxumError, AxumResult},
    middlewares::require_auth::UserData,
    models::{
        organization::Organization,
        user::{ OrganizationRole},
    },
    state::AppState,
};

#[derive(Clone, Debug, Deserialize)]
pub struct Params {
    org_id: ObjectId,
}

#[derive(Clone, Debug, Serialize, ToSchema, Deserialize)]
pub struct MembershipData(pub crate::models::user::Membership);

#[derive(Clone, Debug, Serialize, ToSchema, Deserialize)]
pub struct OrganizationData(pub Organization);

pub async fn require_org_membership(
    Extension(state): Extension<AppState>,
    Extension(user_data): Extension<UserData>,
    Path(Params { org_id }): Path<Params>,
    mut request: Request,
    next: Next,
) -> AxumResult<Response> {
    let org_data = state
        .store
        .organization
        .get_by_id(org_id)
        .await?
        .ok_or_else(|| AxumError::not_found(eyre!("Organization not found")))?;

    let is_member = org_data
        .members
        .iter()
        .find(|m| m.user_id == user_data.0.id);

    if is_member.is_none() {
        return Err(AxumError::forbidden(eyre!(
            "You are not a member of this organization"
        )));
    }

    request
        .extensions_mut()
        .insert(MembershipData(is_member.unwrap().clone()));

    request
        .extensions_mut()
        .insert(OrganizationData(org_data.clone()));

    Ok(next.run(request).await)
}

pub async fn requre_org_admin(
    Extension(membership): Extension<MembershipData>,
    request: Request,
    next: Next,
) -> AxumResult<Response> {
    if membership.0.role != OrganizationRole::Admin {
        return Err(AxumError::forbidden(eyre!(
            "You are not an admin of this organization"
        )));
    }

    Ok(next.run(request).await)
}
