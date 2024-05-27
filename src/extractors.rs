// use axum::{
//     async_trait,
//     extract::{FromRequest, FromRequestParts, Request, State},
//     http::{request::Parts, StatusCode},
//     response::Response,
// };
// use tower_sessions::Session;
//
// use crate::{users::User, AppState};
//
// pub struct AuthenticatedUser(Option<User>);
//
// #[async_trait]
// impl<S> FromRequestParts<S> for AuthenticatedUser
// where
//     S: Send + Sync,
// {
//     #[doc = r#" If the extractor fails it'll use this "rejection" type. A rejection is"#]
//     #[doc = r" a kind of error that can be converted into a response."]
//     type Rejection = (StatusCode, &'static str);
//
//     async fn from_request_parts(req: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
//         let State(state) = State::from_request_parts(req, state)
//             .await
//             .map_err(|e| (StatusCode::from_u16(500), "failed to get state"))?;
//         let session = Session::from_request_parts(req, state).await?;
//
//         let user = match session.get::<u32>("user_id").await {
//             Ok(Some(user_id)) => {}
//             _ => None,
//         };
//
//         Ok(AuthenticatedUser(Some(user)))
//     }
// }
