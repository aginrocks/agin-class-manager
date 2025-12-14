use axum::{Extension, extract::Request, middleware::Next, response::Response};
use axum_oidc::OidcClaims;
use color_eyre::eyre::{self, ContextCompat};
use sea_orm::ActiveValue::Set;
use sea_orm::sea_query::OnConflict;
use sea_orm::{EntityTrait, ModelTrait};
use serde::Serialize;
use tower_sessions::Session;
use utoipa::ToSchema;

use crate::models::organization;
use crate::{
    axum_error::{AxumError, AxumResult},
    middlewares::GroupClaims,
    models::user::{self},
    state::AppState,
};

/// Middleware that ensures the user is authenticated
pub async fn require_auth(
    claims: Option<OidcClaims<GroupClaims>>,
    Extension(state): Extension<AppState>,
    _session: Session,
    mut request: Request,
    next: Next,
) -> AxumResult<Response> {
    let claims = claims.ok_or_else(|| AxumError::unauthorized(eyre::eyre!("Unauthorized")))?;

    let sub = claims.subject().to_string();
    let name = claims
        .name()
        .wrap_err("Name is required")?
        .get(None)
        .wrap_err("Name is required")?
        .to_string();
    let email = claims.email().wrap_err("Email is required")?.to_string();

    let user_model = user::ActiveModel {
        email: Set(email),
        name: Set(name),
        subject: Set(sub),
        ..Default::default()
    };

    let user = user::Entity::insert(user_model)
        .on_conflict(
            OnConflict::column(user::Column::Subject)
                .update_columns([user::Column::Email, user::Column::Name])
                .to_owned(),
        )
        .exec_with_returning(&state.sea_orm)
        .await?;

    let orgs = user
        .find_related(organization::Entity)
        .all(&state.sea_orm)
        .await?;

    request.extensions_mut().insert(user);
    request.extensions_mut().insert(orgs);

    Ok(next.run(request).await)
}

#[derive(Serialize, ToSchema)]
#[schema(example = json!({"error": "Unauthorized"}))]
pub struct UnauthorizedError {
    error: String,
}
