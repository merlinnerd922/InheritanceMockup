use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub struct Dvs {

}

impl Display for Dvs {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "")
  }
}