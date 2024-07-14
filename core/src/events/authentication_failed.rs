use crate::models::user::UserId;

#[derive(Clone)]
pub struct AuthenticationFailed(pub UserId);

impl_event!(AuthenticationFailed);
