use crate::models::Ingredient;

#[derive(Debug)]
pub struct RecipeItem {
    amount: i8,
    unit: RecipeItemUnit,
    ingredient: Box<dyn Ingredient>,
    specification: Option<String>,
}

impl RecipeItem {
    pub fn new(
        amount: i8,
        unit: RecipeItemUnit,
        ingredient: Box<dyn Ingredient>,
        specification: Option<String>,
    ) -> Self {
        Self {
            ingredient,
            amount,
            unit,
            specification,
        }
    }
}

#[derive(Debug)]
pub enum RecipeItemUnit {
    Unit,
    Cup,
    Tablespoon,
    Teaspoon,
    OzCan,
    Sprinkle,
}

#[cfg(test)]
pub mod mocks {
    use super::*;
    use crate::models::ScalarIngredient;

    impl RecipeItem {
        pub fn mock() -> Self {
            Self::new(
                2,
                RecipeItemUnit::Cup,
                Box::new(ScalarIngredient::mock()),
                Some(String::from("lightly chopped")),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Recipe, ScalarIngredient};

    #[test]
    fn recipe_item_mock() -> crate::Result<()> {
        RecipeItem::mock();
        Ok(())
    }

    #[test]
    fn recipe_item_new_with_scalar() -> crate::Result<()> {
        RecipeItem::new(
            2,
            RecipeItemUnit::Cup,
            Box::new(ScalarIngredient::mock()),
            Some(String::from("lightly chopped")),
        );
        Ok(())
    }

    #[test]
    fn recipe_item_new_with_recipe() -> crate::Result<()> {
        RecipeItem::new(
            2,
            RecipeItemUnit::Cup,
            Box::new(Recipe::mock()),
            Some(String::from("lightly chopped")),
        );
        Ok(())
    }
}
