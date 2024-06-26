use diesel::query_builder::BoxedSqlQuery;
use diesel::{QueryDsl, SelectableHelper};

use super::Repository;
use crate::schema;
use crate::models::nuisance_family::{InsertNuisanceFamily, NuisanceFamily, NuisanceFamilyId};

pub trait NuisanceFamilyRepository {
    type Error;

    /// Insère une famille de nuisance dans le repertoire.
    fn insert_nuisance_family(&mut self, insert: InsertNuisanceFamily) -> Result<NuisanceFamilyId, Self::Error>;


    fn delete_nuisance_family(&mut self, id: NuisanceFamilyId)
    
}


impl NuisanceFamilyRepository for Repository {
    type Error = diesel::result::Error;

    /// Insère une famille de nuisance dans le repertoire.
    fn insert_nuisance_family(&mut self, insert: InsertNuisanceFamily) -> NuisanceFamilyId {
        use crate::schema::nuisance_families;

        diesel::insert_into(nuisance_families::table)
            .values(insert)
            .returning(nuisance_families::id)
            .get_result(self.conn)
    }


}
