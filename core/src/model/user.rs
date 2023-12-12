pub struct RegisterUser {
    pub name:       String,
    pub email:      String,
    pub password:   String
}

#[derive(Default)]
pub struct UserFilter {
    pub name_or_email: Option<String>
}

impl UserFilter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name_or_email<V: Into<String>>(mut self, value: V) -> Self {
        self.name_or_email = Option::Some(value.into());
        self
    }
}

pub struct User {
    pub id:     String,
    pub name:   String,
    pub email:  String,
    pub avatar: Option<String>
}
