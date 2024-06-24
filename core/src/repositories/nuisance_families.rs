use diesel::SelectableHelper;

use super::Repository;
use crate::models::nuisance::{InsertNuisanceFamily, NuisanceFamily};

impl Repository {
    /// Insère une famille de nuisance dans le repository.
    pub fn insert_nuisance_family(&mut self, family: &InsertNuisanceFamily) {
        use crate::schema::nuisance_families;

        diesel::insert_into(nuisance_families::table)
            .values(family)
            .returning(NuisanceFamily::as_returning())
            .get_result(self.conn)
    }
}
