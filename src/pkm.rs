use crate::dvs::Dvs;
use crate::nature::Nature;
use crate::species_index::SpeciesIndex;
use boilermates::boilermates;


#[boilermates("PK1", "PK3", "PK7")]
#[derive(Debug)]
pub struct Pkm {
    pub name: String,
    pub level: i32,
    pub current_exp: i32,
    pub species_index: SpeciesIndex,

    #[boilermates(only_in("PK7"))]
    pub affection: i32,

    #[boilermates(not_in("PK1"))]
    pub nature: Nature,

    #[boilermates(only_in("PK1"))]
    pub dvs: Dvs,

    #[boilermates(only_in("PK3", "PK7"))]
    pub is_shiny: bool,
}
