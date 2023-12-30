pub mod client;
pub mod user;
pub mod session;
pub mod credentials;
pub mod log;
pub mod report;

pub trait Identifiable {
    type Type;

    fn id(&self) -> Self::Type;
}