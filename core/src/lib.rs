#[cfg(feature = "backend")]
mod crypto;

pub mod error;
pub mod issues;
pub mod validation;

pub mod forms;
pub mod models;

#[cfg(feature = "backend")]
pub mod repositories;

//pub mod config;
#[cfg(feature = "backend")]
pub mod events;

#[cfg(feature = "backend")]
pub mod services;

//pub use log;

#[cfg(feature = "backend")]
mod backend {
    use crate::events::EventBus;
    use crate::services::account::Account;
    use crate::services::authentication::Authentication;
    use crate::services::reporting::Reporting;

    use crate::repositories::{Repository, RepositorySettings};
    use crate::services::ServiceSettings;

    #[derive(Default, Clone)]
    /// Paramètres de configuration pour Signuis.
    pub struct SgSettings {
        service: ServiceSettings,
        repos: RepositorySettings,
    }

    impl SgSettings {
        pub fn set_max_connections(&mut self, value: u32) -> &mut Self {
            self.repos.set_max_connections(value);
            self
        }
    }

    #[cfg(feature = "backend")]
    #[derive(Clone)]
    /// Système principal de Signuis,
    pub struct Signuis {
        pub reporting: Reporting,
        /// Service de gestion de l'authentification
        pub auth: Authentication,
        /// Service de gestion des comptes utilisateurs
        pub account: Account,
        /// Répertoires de données
        pub repos: Repository,
        /// Bus évènementiel
        pub events: EventBus,
    }

    #[cfg(feature = "backend")]
    impl Signuis {
        pub async fn new(settings: SgSettings) -> Result<Self, crate::error::Error> {
            let events = EventBus::new();
            let repos = Repository::new(&settings.repos).await?;
            let account = Account::new(repos.clone(), events.clone());
            let auth = Authentication::new(repos.clone(), events.clone());
            let reporting = Reporting::new(repos.clone(), events.clone());

            let repos = Repository::new(&settings.repos).await?;

            Ok(Self {
                reporting,
                auth,
                account,
                repos,
                events,
            })
        }
    }
}

#[cfg(feature = "backend")]
pub use backend::*;
