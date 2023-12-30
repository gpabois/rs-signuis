use chrono::{DateTime, Utc};
use uuid::Uuid;
use super::{session::Session, client::Client};

pub type LogClient = Client;

pub struct Log {
    pub r#type:  String,
    pub message: Option<String>,
    pub args:    Option<String>,
    pub client:  Option<LogClient>,
    pub user_id: Option<Uuid>,
    pub at:      DateTime<Utc>
}

#[derive(Clone)]
pub struct NewLog {
    pub r#type: String,
    pub client: Option<Client>,
    pub message: Option<String>,
    pub args:    Option<String>,
    pub user_id: Option<Uuid>,
}

impl NewLog {
    pub fn new(r#type: &str) -> Self {
        Self {
            r#type: r#type.into(), 
            client: None, 
            message: None,
            args: None,
            user_id: None
        }
    }

    // Log event emitted by an actor (session)
    pub fn from_actor(&mut self, from: &Session) -> &mut Self {
        self.client = Some(from.client.clone());
        self.user_id = from.user.as_ref().map(|user| user.id.clone());
        self
    }
}

impl Into<InsertLog> for NewLog {
    fn into(self) -> InsertLog {
        InsertLog {
            id: None,
            r#type:  self.r#type,
            client:  self.client,
            message: self.message,
            args:    self.args,
            user_id: self.user_id,
            at:      None
        }
    }
}

pub struct InsertLog {
    pub id: Option<Uuid>,
    pub r#type: String,
    pub client: Option<Client>,
    pub message: Option<String>,
    pub args:    Option<String>,
    pub user_id: Option<Uuid>,
    pub at:      Option<DateTime<Utc>>
}

impl InsertLog {
    pub fn new(r#type: &str) -> Self {
        Self {
            id: None,
            r#type: r#type.into(),
            client: None,
            message: None,
            args: None,
            user_id: None,
            at: None
        }
    }

    pub fn set_id(&mut self, id: Uuid) -> &mut Self {
        self.id = Some(id);
        self
    }

    pub fn set_type(&mut self, value: String) -> &mut Self {
        self.r#type = value;
        self
    }

    pub fn set_client(&mut self, value: Client) -> &mut Self {
        self.client = Some(value);
        self
    }

    pub fn set_message(&mut self, value: String) -> &mut Self {
        self.message = Some(value);
        self
    }

    pub fn set_args(&mut self, value: String) -> &mut Self {
        self.args = Some(value);
        self
    }

    pub fn set_user_id(&mut self, value: Uuid) -> &mut Self {
        self.user_id = Some(value);
        self
    }

    pub fn set_at(&mut self, value: DateTime<Utc> ) -> &mut Self {
        self.at = Some(value);
        self
    }
}

pub enum LogFilter {
    Or(Vec<LogFilter>),
    And(Vec<LogFilter>),
    TypeEq(String),
    UserIdEq(Uuid),
    ClientIpEq(String),
    AtGte(DateTime<Utc>)
}
