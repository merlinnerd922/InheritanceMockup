use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub enum SpeciesIndex {
  BULBASAUR = 1,
  GIRATINA = 2,
}

impl Display for SpeciesIndex {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}