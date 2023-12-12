use super::user::User;


pub struct Session {
    pub id:     String,
    pub user:   Option<User>,
    pub ip:     String
}

impl Session {
    pub fn from_ip(data: (String, String)) -> Self {
        return Session {
            id: data.0,
            ip: data.1,
            user: Option::None
        }
    }
}
