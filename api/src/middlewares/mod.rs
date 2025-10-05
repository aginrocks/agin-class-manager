use serde::{Deserialize, Serialize};

pub mod require_auth;
pub mod require_class_permissions;

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct GroupClaims {}
impl axum_oidc::AdditionalClaims for GroupClaims {}
impl openidconnect::AdditionalClaims for GroupClaims {}
