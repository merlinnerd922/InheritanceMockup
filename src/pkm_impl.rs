use std::fmt;
use std::fmt::Display;
use crate::ohpkm::OHPKM;
use crate::pkm::{PK1, PK3, PK7};

impl PK1 {
    pub(crate) fn new(ohpkm: OHPKM) -> PK1 {
        PK1 {
            name: ohpkm.name,
            level: ohpkm.level,
            current_exp: ohpkm.current_exp,
            species_index: ohpkm.species_index,
            dvs: ohpkm.dvs,
        }
    }
}

impl Display for PK1 {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({}, {}, {}, {}, {}) ", self.name, self.species_index, self.current_exp, self.level, self.dvs)
  }
}


impl PK3 {
  pub(crate) fn new(ohpkm: OHPKM) -> PK3 {
    PK3 {
      name: ohpkm.name,
      level: ohpkm.level,
      current_exp: ohpkm.current_exp,
      species_index: ohpkm.species_index,
      nature: ohpkm.nature,
      is_shiny: ohpkm.is_shiny,
    }
  }
}

impl Display for PK3 {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({}, {}, {}, {}, {}) ", self.name, self.species_index, self.current_exp, self.level, self.nature)
  }
}

impl PK7 {
  pub(crate) fn new(ohpkm: OHPKM) -> PK7 {
    PK7 {
      name: ohpkm.name,
      level: ohpkm.level,
      current_exp: ohpkm.current_exp,
      species_index: ohpkm.species_index,
      affection: ohpkm.affection,
      nature: ohpkm.nature,
      is_shiny: ohpkm.is_shiny,
    }
  }
}

impl Display for PK7 {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({}, {}, {}, {}, {}, {}) ", self.name, self.species_index, self.current_exp, 
           self.level, self.nature, self.affection)
  }
}