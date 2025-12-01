use axum::{Extension, Json, extract::State, middleware};
use axum_valid::Valid;
use color_eyre::eyre::eyre;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    axum_error::{AxumError, AxumResult},
    middlewares::{
        require_auth::UnauthorizedError,
        require_org_permissions::{OrganizationData, requre_org_admin},
    },
    models::santa::{MutableSanta, PartialSanta, PartialSantaParticipant, PopulatedSanta},
    routes::api::CreateSuccess,
    state::AppState,
};

pub fn routes() -> OpenApiRouter<AppState> {
    let admin = OpenApiRouter::new()
        .routes(routes!(create_secret_santa))
        .layer(middleware::from_fn(requre_org_admin));

    let user = OpenApiRouter::new().routes(routes!(get_secret_santa));

    admin.merge(user)
}

/// Initialize secret santa
#[utoipa::path(
    method(post),
    path = "/",
    request_body = MutableSanta,
    responses(
        (status = OK, description = "Success", body = CreateSuccess),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json")
    ),
    tag = "Secret santa"
)]
#[axum::debug_handler]
async fn create_secret_santa(
    State(state): State<AppState>,
    Extension(org_data): Extension<OrganizationData>,
    Valid(body): Valid<Json<MutableSanta>>,
) -> AxumResult<Json<CreateSuccess>> {
    let mut participants = vec![];
    for participant in &body.participants {
        if org_data
            .0
            .members
            .iter()
            .any(|member| member.user_id != participant.user_id)
        {
            return Err(AxumError::bad_request(eyre!(
                "All participants must be in the organization"
            )));
        }

        let oid = state
            .store
            .santa_participant
            .create(PartialSantaParticipant {
                present_reciever: participant.present_reciever,
                user_id: participant.user_id,
                proposition: participant.proposition.clone(),
            })
            .await?;

        participants.push(oid.id);
    }

    let santa = state
        .store
        .santa
        .create(PartialSanta {
            organization_id: org_data.0.id,
            participants,
            propositions_due: body.propositions_due,
            start_date: body
                .start_date
                .map(|dt| dt.into())
                .unwrap_or_else(|| chrono::Utc::now().into()),
            end_date: body
                .end_date
                .map(|dt| dt.into())
                .unwrap_or_else(|| chrono::Utc::now().into()),
        })
        .await?;

    Ok(Json(CreateSuccess {
        success: true,
        id: santa.id.to_string(),
    }))
}

/// Get a secret santa for an organization
#[utoipa::path(
    method(get),
    path = "/",
    responses(
        (status = OK, description = "Success", body = PopulatedSanta, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json")
    ),
    tag = "Secret santa"
)]
async fn get_secret_santa(
    Extension(state): Extension<AppState>,
    Extension(org_data): Extension<OrganizationData>,
) -> AxumResult<Json<PopulatedSanta>> {
    let santa = state
        .store
        .santa
        .get_by_organization(&org_data.0.id)
        .await?
        .populate_participants(state)
        .await?;

    Ok(Json(santa))
}
