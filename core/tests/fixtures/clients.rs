use fake::{faker::internet::fr_fr::{IP, UserAgent}, Fake};
use signuis_core::model::session::Client;

pub fn new_client() -> Client {
    let ip: String = IP().fake();
    let user_agent: String = UserAgent().fake();

    Client::new(&ip, &user_agent)
}
