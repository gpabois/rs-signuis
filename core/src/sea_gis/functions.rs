use sea_query::{Func, FunctionCall, IntoIden};

use super::Geometry;

/// Retourne la distance entre deux géométries
pub fn st_distance<G1, G2>(geom1: G1, geom2: G2) -> FunctionCall 
where G1: IntoIden, G2: IntoIden 
{
    Func::cust(ST_Distance).args((
        geom1,
        geom2
    ))
}

/// Vérifie si geom1 est complètement dans geom2
pub fn st_within<G1, G2>(geom1: G1, geom2: G2) -> FunctionCall 
where G1: IntoIden, G2: IntoIden 
{
    Func::cust(ST_Within).args((
        geom1,
        geom2
    ))
}


#[derive(Iden)]
#[iden = "ST_Distance"]
pub struct ST_Distance;


#[derive(Iden)]
#[iden = "ST_Within"]
pub struct ST_Within;

