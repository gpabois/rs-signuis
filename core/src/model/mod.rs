pub mod user;
pub mod session;
pub mod credentials;

pub trait Identifiable {
    type Type;

    fn id(&self) -> Self::Type;
}