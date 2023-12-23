use sea_query::{Iden, DynIden, SimpleExpr, IntoIden};
pub struct ConditionalInsert(Vec<(DynIden, SimpleExpr)>);

impl ConditionalInsert {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn add<I>(&mut self, col: I, value: SimpleExpr) -> &mut Self where I: IntoIden {
        self.0.push((col.into_iden(), value));
        self
    }

    pub fn r#if<I, F>(&mut self, test: bool, col: I, value: F) -> &mut Self where I: IntoIden, F: FnOnce() -> SimpleExpr {
        if test {
            self.0.push((col.into_iden(), value()))
        }

        self
    }

    pub fn into_tuple(self) -> (Vec<DynIden>, Vec<SimpleExpr>) {
        let mut t: (Vec<DynIden>, Vec<SimpleExpr>) = (vec![], vec![]);

        self.0.into_iter().for_each(|(c, v)| {
            t.0.push(c);
            t.1.push(v)
        });

        t
    }
}

pub enum SessionIden {
    Table,
    ID,
    UserID,
    Token,
    UserAgent,
    IP,
    CreatedAt,
    ExpiresAt
}

impl Iden for SessionIden {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(s, "{}", match self {
            Self::Table => "sessions",
            Self::ID => "id",
            Self::UserID => "user_id",
            Self::Token => "token",
            Self::UserAgent => "user_agent",
            Self::IP => "ip",
            Self::CreatedAt => "created_at",
            Self::ExpiresAt => "expires_at"
        }).unwrap();
    }
}

pub enum UserIden {
    Table,
    ID,
    Name,
    Email,
    Avatar,
    Password
}

impl Iden for UserIden {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(s, "{}", match self {
            Self::Table => "users",
            Self::ID => "id",
            Self::Name => "name",
            Self::Email => "email",
            Self::Avatar => "avatar",
            Self::Password => "password"
        }).unwrap();
    }
}
