use sea_query::Iden;

pub enum SessionIden {
    Table,
    ID,
    UserID,
    Token,
    UserAgent,
    IP,
    //CreatedAt,
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
            //Self::CreatedAt => "created_at",
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
