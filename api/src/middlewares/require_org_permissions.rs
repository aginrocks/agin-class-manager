use axum::{
    Extension,
    extract::{Path, Request, State},
    middleware::Next,
    response::Response,
};
use color_eyre::eyre::eyre;
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use tower_sessions::Session;
use utoipa::ToSchema;

use crate::{
    axum_error::{AxumError, AxumResult},
    middlewares::require_auth::UserData,
    models::organization::Organization,
    state::AppState,
};

#[derive(Clone, Debug, Deserialize)]
struct Params {
    org_id: String,
}

#[derive(Clone, Debug, Serialize, ToSchema, Deserialize)]
pub struct MembershipData(pub crate::models::user::Membership);

pub async fn require_org_membership(
    State(state): State<AppState>,
    Extension(user_data): Extension<UserData>,
    session: Session,
    mut request: Request,
    Path(Params { org_id }): Path<Params>,
    next: Next,
) -> AxumResult<Response> {
    let org_data = state
        .store
        .organization
        .get_by_id(&org_id)
        .await?
        .ok_or_else(|| AxumError::not_found(eyre!("Class not found")))?;

    let is_member = org_data
        .members
        .iter()
        .find(|m| m.user_id == user_data.0.id);

    if is_member.is_none() {
        return Err(AxumError::forbidden(eyre!(
            "You are not a member of this class"
        )));
    }

    request
        .extensions_mut()
        .insert(MembershipData(is_member.unwrap().clone()));

    Ok(next.run(request).await)
}
