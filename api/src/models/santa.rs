use crate::database_object;
use crate::mongo_id::{object_id_as_string_required, vec_oid_to_vec_string};
use crate::state::AppState;
use bson::doc;
use chrono::Utc;
use color_eyre::eyre::Result;
use futures::stream::TryStreamExt;
use mongodb::bson::oid::ObjectId;
use partial_struct::Partial;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;
use visible::StructFields;

database_object!(SantaParticipant {
    #[schema(value_type = String)]
    #[serde(with = "object_id_as_string_required")]
    user_id: ObjectId,
    #[schema(value_type = String)]
    #[serde(with = "object_id_as_string_required")]
    present_reciever: ObjectId,
    proposition: String,
});

database_object!(Santa {
    #[serde(
        rename = "_id",
        with = "object_id_as_string_required"
    )]
    #[schema(value_type = String)]
    id: ObjectId,
    start_date: mongodb::bson::DateTime,
    propositions_due: Option<mongodb::bson::DateTime>,
    end_date: mongodb::bson::DateTime,
    #[serde(with = "vec_oid_to_vec_string")]
    #[schema(value_type = Vec<SantaParticipant>)]
    participants: Vec<ObjectId>,
    #[schema(value_type = String)]
    #[serde(with = "object_id_as_string_required")]
    organization_id: ObjectId,
});

#[derive(Serialize, Deserialize, ToSchema, Validate)]
#[StructFields(pub)]
pub struct MutableSanta {
    participants: Vec<SantaParticipant>,
    start_date: Option<chrono::DateTime<Utc>>,
    propositions_due: Option<mongodb::bson::DateTime>,
    end_date: Option<chrono::DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, ToSchema, Validate)]
#[StructFields(pub)]
pub struct PopulatedSanta {
    #[serde(rename = "_id", with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    id: ObjectId,
    start_date: mongodb::bson::DateTime,
    propositions_due: Option<mongodb::bson::DateTime>,
    end_date: mongodb::bson::DateTime,
    participants: Vec<SantaParticipant>,
    #[schema(value_type = String)]
    #[serde(with = "object_id_as_string_required")]
    organization_id: ObjectId,
}

impl Santa {
    pub async fn populate_participants(&self, state: AppState) -> Result<PopulatedSanta> {
        let participants = state.database.collection::<SantaParticipant>("santa-participants").find(
            doc! {"_id": {"$in": self.participants.iter().map(|g| g.to_owned()).collect::<Vec<_>>() }},
        ).await?;

        let participants: Vec<SantaParticipant> = participants.try_collect().await?;

        let res = PopulatedSanta {
            id: self.id,
            organization_id: self.organization_id,
            start_date: self.start_date,
            propositions_due: self.propositions_due,
            end_date: self.end_date,
            participants,
        };
        Ok(res)
    }
}
