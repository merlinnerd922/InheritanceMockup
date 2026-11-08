use crate::dvs::Dvs;
use crate::nature::Nature;
use crate::species_index::SpeciesIndex;

#[derive(Clone)]
pub struct OHPKM {
  pub name: String,
  pub level: i32,
  pub current_exp: i32,
  pub species_index : SpeciesIndex,
  pub affection : i32,
  pub dvs: Dvs,
  pub is_shiny: bool,
  pub nature: Nature
}