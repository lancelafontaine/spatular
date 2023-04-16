use crate::models::Ingredient;

pub struct ScalarIngredient {
    name: String,
}

impl ScalarIngredient {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl Ingredient for ScalarIngredient {}

#[cfg(test)]
pub mod mocks {
    use super::*;

    impl ScalarIngredient {
        pub fn mock() -> Self {
            Self {
                name: String::from("kale"),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scalar_ingredient_mock() -> crate::Result<()> {
        ScalarIngredient::mock();
        Ok(())
    }

    #[test]
    fn scalar_ingredient_new() -> crate::Result<()> {
        ScalarIngredient::new(String::from("kale"));
        Ok(())
    }
}
