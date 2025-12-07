use axum::{
    Extension,
    extract::{Path, Request},
    middleware::Next,
    response::Response,
};
use color_eyre::eyre::eyre;
use mongodb::bson::{doc, oid::ObjectId};
use sea_orm::{ColumnTrait, ModelTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    axum_error::{AxumError, AxumResult},
    middlewares::require_auth::UserData,
    models::{
        self,
        organization::{self, Organization},
    },
    state::AppState,
};

#[derive(Clone, Debug, Deserialize)]
pub struct Params {
    org_id: i64,
}

#[derive(Clone, Debug, Serialize, ToSchema, Deserialize)]
pub struct MembershipData(pub crate::models::user::Membership);

#[derive(Clone, Debug, Serialize, ToSchema, Deserialize)]
pub struct OrganizationData(pub Organization);

pub async fn require_org_membership(
    Extension(state): Extension<AppState>,
    Extension(organizations): Extension<Vec<organization::Model>>,
    Extension(user): Extension<models::user::Model>,
    Path(Params { org_id }): Path<Params>,
    mut request: Request,
    next: Next,
) -> AxumResult<Response> {
    // let org_data = state
    //     .store
    //     .organization
    //     .get_by_id(org_id)
    //     .await?
    //     .ok_or_else(|| AxumError::not_found(eyre!("Organization not found")))?;

    // let is_member = org_data
    //     .members
    //     .iter()
    //     .find(|m| m.user_id == user_data.0.id);

    let org = organizations.iter().find(|org| org.id == org_id);

    let org = if let Some(org) = org {
        org
    } else {
        return Err(AxumError::not_found(eyre!("Organization not found")));
    };

    let member = org
        .find_related(models::user::Entity)
        .filter(models::user::Column::Id.eq(user.id))
        .one(&state.sea_orm)
        .await?;

    if member.is_none() {
        return Err(AxumError::forbidden(eyre!(
            "You are not a member of this organization"
        )));
    }

    // request
    //     .extensions_mut()
    //     .insert(MembershipData(is_member.unwrap().clone()));

    // request
    //     .extensions_mut()
    //     .insert(OrganizationData(org_data.clone()));

    Ok(next.run(request).await)
}

// pub async fn requre_org_admin(
//     Extension(membership): Extension<MembershipData>,
//     request: Request,
//     next: Next,
// ) -> AxumResult<Response> {
//     if membership.0.role != OrganizationRole::Admin {
//         return Err(AxumError::forbidden(eyre!(
//             "You are not an admin of this organization"
//         )));
//     }

//     Ok(next.run(request).await)
// }
