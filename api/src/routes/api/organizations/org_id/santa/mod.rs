use axum::{Extension, Json, extract::State, middleware};
use axum_valid::Valid;
use color_eyre::eyre::eyre;
use sea_orm::{ActiveValue::Set, EntityTrait, ModelTrait};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    axum_error::{AxumError, AxumResult},
    middlewares::{require_auth::UnauthorizedError, require_org_permissions::requre_org_admin},
    models::{
        organization,
        santa::{self, MutableSanta, PopulatedSanta},
        santa_participants::{self, SantaParticipant},
        user,
    },
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
    params(
        ("org_id" = i64, Path, description = "Organization id"),
    ),
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
    Extension(org_data): Extension<organization::Model>,
    Valid(body): Valid<Json<MutableSanta>>,
) -> AxumResult<Json<CreateSuccess>> {
    let santa_model = santa::ActiveModel {
        start_date: Set(body.start_date.unwrap_or_else(|| chrono::Utc::now())),
        organization_id: Set(org_data.id),
        propositions_due: Set(body.propositions_due),
        end_date: Set(body.end_date.unwrap_or_else(|| {
            chrono::Utc::now()
                .checked_add_months(chrono::Months::new(1))
                .ok_or_else(|| eyre!("Can't set end_date"))
                .unwrap()
        })),
        ..Default::default()
    };

    let santa = santa::Entity::insert(santa_model)
        .exec_with_returning(&state.sea_orm)
        .await?;

    let mut participants = vec![];
    for participant in &body.participants {
        if !org_data
            .find_related(user::Entity)
            .all(&state.sea_orm)
            .await?
            .iter()
            .any(|member| member.id == *participant)
        {
            return Err(AxumError::bad_request(eyre!(
                "All participants must be in the organization"
            )));
        }

        let participant_model = santa_participants::ActiveModel {
            user_id: Set(*participant),
            santa_id: Set(santa.id),
            ..Default::default()
        };

        participants.push(participant_model);
    }

    santa_participants::Entity::insert_many(participants)
        .exec(&state.sea_orm)
        .await?;

    Ok(Json(CreateSuccess {
        success: true,
        id: santa.id,
    }))
}

/// Get a secret santa for an organization
#[utoipa::path(
    method(get),
    path = "/",
    params(
        ("org_id" = i64, Path, description = "Organization id"),
    ),
    responses(
        (status = OK, description = "Success", body = PopulatedSanta, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json")
    ),
    tag = "Secret santa"
)]
async fn get_secret_santa(
    Extension(state): Extension<AppState>,
    Extension(org_data): Extension<organization::Model>,
) -> AxumResult<Json<PopulatedSanta>> {
    let Some(santa) = santa::Entity::find_by_organization_id(org_data.id)
        .one(&state.sea_orm)
        .await?
    else {
        return Err(AxumError::not_found(eyre!(
            "No secret santa event found for this organization"
        )));
    };

    let santa_participants = santa
        .find_related(user::Entity)
        .select_also(santa_participants::Entity)
        .all(&state.sea_orm)
        .await?;

    let participants: Vec<SantaParticipant> = santa_participants
        .into_iter()
        .filter_map(|(user, participant)| {
            Some(SantaParticipant {
                id: user.id,
                email: user.email,
                name: user.name,
                receiver: participant.clone()?.receiver_id,
                proposition: participant?.proposition,
            })
        })
        .collect();

    Ok(Json(PopulatedSanta {
        id: santa.id,
        end_date: santa.end_date,
        organization_id: santa.organization_id,
        start_date: santa.start_date,
        propositions_due: santa.propositions_due,
        participants,
    }))
}
