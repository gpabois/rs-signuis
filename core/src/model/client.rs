
#[derive(Clone)]
pub struct Client {
    pub ip: String,
    pub user_agent: String
}

impl Client {
    pub fn new(ip: &str, user_agent: &str) -> Self {
        Self{
            ip: ip.into(),
            user_agent: user_agent.into()
        }
    }
}