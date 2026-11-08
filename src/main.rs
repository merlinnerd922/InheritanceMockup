use crate::dvs::Dvs;
use crate::nature::Nature;
use crate::ohpkm::OHPKM;
use crate::pkm::{PK1, PK3, PK7};
use crate::species_index::SpeciesIndex;

mod dvs;
mod nature;
mod ohpkm;
pub mod pkm;
mod pkm_impl;
mod species_index;

fn main() {
    let ohpkm: OHPKM = OHPKM {
        name: String::from("Giratina"),
        level: 50,
        current_exp: 1000000,
        species_index: SpeciesIndex::GIRATINA,
        affection: 0,
        dvs: Dvs {},
        is_shiny: false,
        nature: Nature::Naughty,
    };
    let ohpkm1 = ohpkm.clone();
    let ohpkm2 = ohpkm.clone();

    let pk1 = PK1::new(ohpkm);
    let pk3 = PK3::new(ohpkm1);
    let pk7 = PK7::new(ohpkm2);

    println!("{}", pk1);
    println!("{}", pk3);
    println!("{}", pk7);
}
