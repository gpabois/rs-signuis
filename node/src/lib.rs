use futures::future::BoxFuture;
use node_bindgen::derive::node_bindgen;
use once_cell::sync::Lazy;
use signuis_core::{services::{ServicePool, reporting::traits::Reporting, authentication::traits::Authentication}, entities::{nuisance::{NuisanceReport, CreateNuisanceReport}, session::Session, client::Client}, Error};
use futures::lock::Mutex;

pub struct ServicePoolContainer(Option<ServicePool>);

static SVC_SINGLETON: Lazy<Mutex<ServicePoolContainer>> = Lazy::new(|| Mutex::new(ServicePoolContainer(None)));

impl ServicePoolContainer {
    pub fn get<'a>() -> BoxFuture<'a, Result<ServicePool, signuis_core::Error>> {
        Box::pin(async {
            let mut r = SVC_SINGLETON.lock().await;
        
            if r.0.is_none() {
                *r = ServicePoolContainer(Some(ServicePool::from_config().await?));
            }
    
            Ok(r.0.clone().unwrap())
        })
    }
}

#[node_bindgen]
fn anonymous_session(client: Client) -> Session { Session::anonymous(client) }

#[node_bindgen]
async fn check_session_token(token: String) -> Result<Session, Error> {
    ServicePoolContainer::get()
    .await?
    .with(|tx| 
        Box::pin(async move {
            tx
            .check_session_token(&token)
            .await
        }
    )).await
}


#[node_bindgen]
pub async fn report_nuisance(report: CreateNuisanceReport, actor: Session) -> Result<NuisanceReport, Error> {
    ServicePoolContainer::get()
    .await?
    .with(|tx| Box::pin(async move {
        tx.report_nuisance(report, &actor).await
    }))
    .await
}
