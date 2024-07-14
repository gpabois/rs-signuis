use crate::models::user::UserId;

#[derive(Clone, Copy)]
pub struct UserRegistered(pub UserId);

impl_event!(UserRegistered);
