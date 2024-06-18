use sea_query::{Iden, DynIden, SimpleExpr, IntoIden};

/// Génère une requête d'insertion dynamique dont la structure
/// peut être altérée en fonction de conditions
pub struct ConditionalInsert(Vec<(DynIden, SimpleExpr)>);

impl ToOwned for ConditionalInsert {
    type Owned = Self;

    fn to_owned(&self) -> Self::Owned {
        Self(self.0.to_owned())
    }

    fn clone_into(&self, target: &mut Self::Owned) {
        *target = self.to_owned();
    }
}

impl ConditionalInsert {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Ajoute une colonne avec une valeur associée.
    pub fn add<I>(&mut self, col: I, value: SimpleExpr) -> &mut Self where I: IntoIden {
        self.0.push((col.into_iden(), value));
        self
    }

    /// Si la condition est vraie, ajoute une colonne avec une valeur à insérer
    /// 
    /// # Exemple
    /// L'exemple ajoute une colonne ID à la famille de nuisance, si l'objet définit un identifiant.
    /// 
    /// ```
    /// fn foo(insert: &InsertNuisanceFamily) {
    ///     ConditionalInsert::new()
    ///     .r#if(insert.id.is_some(), NuisanceFamilyIden::ID, || insert.id.unwrap().into());
    /// }

    /// ```
    pub fn r#if<I, F>(&mut self, test: bool, col: I, value: F) -> &mut Self where I: IntoIden, F: FnOnce() -> SimpleExpr {
        if test {
            self.0.push((col.into_iden(), value()))
        }

        self
    }

    pub fn r#if_multi<F>(&mut self, test: bool, value: F) -> &mut Self where F: FnOnce() -> Vec<(DynIden, SimpleExpr)> {
        if test {
            value().into_iter().for_each(|t| self.0.push(t));
        }

        self
    }

    /// Consume l'objet et retourne le tuple (colonnes, valeurs)
    /// qui peuvent être injectés dans un requête SQL.
    /// 
    /// # Exemple
    /// ```
    /// fn foo(insert: ConditionalInsert) -> InsertStatement {
    ///     let (columns, values) = c.into_tuple();
    ///     Query::insert()
    ///         .into_table(NuisanceFamilyIden::Table)
    ///         .columns(columns)
    ///         .values(values)
    ///         .unwrap()
    ///         .to_owned()
    /// }
    /// ```
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
    ClientUserAgent,
    ClientIp,
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
            Self::ClientUserAgent => "client_user_agent",
            Self::ClientIp => "client_ip",
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

pub enum LogIden {
    Table,
    ID,
    Type,
    Message,
    Args,
    At,
    UserID,
    ClientUserAgent,
    ClientIp
}

impl Iden for LogIden {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(s, "{}", match self {
            Self::Table => "logs",
            Self::ID => "id",
            Self::Type => "type",
            Self::Message => "message",
            Self::Args => "args",
            Self::At => "at",
            Self::UserID => "user_id",
            Self::ClientUserAgent => "client_user_agent",
            Self::ClientIp => "client_ip"
        }).unwrap();
    }
}

pub enum NuisanceFamilyIden {
    Table,
    ID,
    Label,
    Description
}

impl Iden for NuisanceFamilyIden {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(s, "{}", match self {
            Self::Table => "nuisance_families",
            Self::ID => "id",
            Self::Label => "label",
            Self::Description => "description"
        }).unwrap();
    }
}


pub enum NuisanceTypeIden {
    Table,
    ID,
    Label,
    Description,
    FamilyId
}

impl Iden for NuisanceTypeIden {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(s, "{}", match self {
            Self::Table => "nuisance_types",
            Self::ID => "id",
            Self::Label => "label",
            Self::Description => "description",
            Self::FamilyId => "family_id"
        }).unwrap();
    }
}

pub enum ReportIden {
    Table,
    ID,
    UserId,
    TypeId,
    Location,
    Intensity,
    CreatedAt
}

impl Iden for ReportIden {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(s, "{}", match self {
            Self::Table => "reports",
            Self::ID => "id",
            Self::UserId => "user_id",
            Self::TypeId => "type_id",
            Self::Intensity => "intensity",
            Self::Location => "location",
            Self::CreatedAt => "created_at"
        }).unwrap();
    }
}
