use diesel::SelectableHelper;

use super::Repository;
use crate::models::nuisance_family::{InsertNuisanceReport, NuisanceReport, NuisanceReportId};

impl Repository {

    /// Insère un nouveau signalement de nuisance dans le répertoire.
    pub fn insert_nuisance_report(&mut self, insert: InsertNuisanceReport) -> NuisanceReportId {
        use crate::schema::nuisance_reports;

        diesel::insert_into(nuisance_reports::table)
            .values(insert)
            .returning(nuisance_reports::id)
            .get_result(self.conn)
    }
}