use crate::{users::User, util::ShatError, AppState};
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
    RequestPartsExt,
};
use tower_sessions::Session;

use super::find_user_by_id;

pub struct AuthenticatedUser(pub User);

#[async_trait]
impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    #[doc = r#" If the extractor fails it'll use this "rejection" type. A rejection is"#]
    #[doc = r" a kind of error that can be converted into a response."]
    type Rejection = ShatError;

    async fn from_request_parts(req: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let session = Session::from_request_parts(req, state)
            .await
            .map_err(|_| ShatError::InternalError)?;
        let state: AppState = req
            .extract_with_state(state)
            .await
            .map_err(|_| ShatError::InternalError)?;

        let user = match session.get::<u32>("user_id").await {
            Ok(Some(user_id)) => find_user_by_id(&state.db, user_id).await,
            _ => None,
        };

        match user {
            Some(user) => Ok(AuthenticatedUser(user)),
            None => Err(ShatError::Unauthorized),
        }
    }
}

pub struct OptionalUser(pub Option<User>);

#[async_trait]
impl<S> FromRequestParts<S> for OptionalUser
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    #[doc = r#" If the extractor fails it'll use this "rejection" type. A rejection is"#]
    #[doc = r" a kind of error that can be converted into a response."]
    type Rejection = ShatError;

    async fn from_request_parts(req: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let session = Session::from_request_parts(req, state)
            .await
            .map_err(|_| ShatError::InternalError)?;
        let state: AppState = req
            .extract_with_state(state)
            .await
            .map_err(|_| ShatError::InternalError)?;

        let user = match session.get::<u32>("user_id").await {
            Ok(Some(user_id)) => find_user_by_id(&state.db, user_id).await,
            _ => None,
        };

        Ok(OptionalUser(user))
    }
}
