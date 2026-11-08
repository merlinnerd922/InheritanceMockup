use std::fmt::{Display, Formatter};

#[derive(Clone)]
#[derive(Debug)]
pub enum Nature {
  Naughty = 0
}

impl Display for Nature {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}