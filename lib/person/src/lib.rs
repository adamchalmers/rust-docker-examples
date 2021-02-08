use serde::{Serialize, Deserialize};
use std::fmt;

#[derive(Serialize, Deserialize)]
pub struct Person{
    pub name: String,
    pub height_cm: u8,
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} is {}cm tall", self.name, self.height_cm)
    }
}