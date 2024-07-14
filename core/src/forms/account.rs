use serde::{Deserialize, Serialize};

use crate::validation::{Validation, Validator};

#[derive(Deserialize, Serialize)]
/// Objet pour enregistrer un nouvel utilisateur.
pub struct RegisterUserForm {
    pub username: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}

impl Validation for RegisterUserForm {
    fn assert(&self, validator: &mut Validator) {
        validator.assert_valid_email(
            &self.email,
            Some("l'adresse courriel est invalide"),
            ["email"],
        );
        validator.assert_eq(
            &self.password,
            &self.confirm_password,
            Some("les mots de passe ne sont pas égaux"),
            ["confirm_password"],
        );
        validator.assert_not_empty(
            &self.password,
            Some("le mot de passe ne doit pas être vide"),
            ["password"],
        );
        validator.assert_not_empty(
            &self.username,
            Some("le nom d'utilisateur ne doit pas être vide"),
            ["username"],
        );
    }
}
