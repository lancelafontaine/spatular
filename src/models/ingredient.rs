use std::fmt::{Debug, Formatter, Result};

pub trait Ingredient {}

impl Debug for dyn Ingredient {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Ingredient").finish()
    }
}
