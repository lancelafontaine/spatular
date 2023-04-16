use crate::models::{Ingredient, RecipeItem, RecipePhoto};
use std::time::Duration;

#[derive(Debug)]
pub struct Recipe {
    items: Vec<RecipeItem>,
    photos: Vec<RecipePhoto>,
    directions: String,
    description: Option<String>,
    num_servings: Option<i8>,
    prep_time: Option<Duration>,
    cook_time: Option<Duration>,
    total_time: Option<Duration>,
}

impl Recipe {
    fn new(items: Vec<RecipeItem>, photos: Vec<RecipePhoto>, directions: String) -> Self {
        Self {
            items,
            photos,
            directions,
            description: None,
            num_servings: None,
            prep_time: None,
            cook_time: None,
            total_time: None,
        }
    }
}

impl Ingredient for Recipe {}

#[cfg(test)]
mod mocks {
    use super::*;

    impl Recipe {
        pub fn mock() -> Self {
            let recipe_items = vec![RecipeItem::mock()];
            let recipe_photos = vec![RecipePhoto::mock()];
            let directions = String::from("bake it");
            Self::new(recipe_items, recipe_photos, directions)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recipe_mock() -> crate::Result<()> {
        Recipe::mock();
        Ok(())
    }

    #[test]
    fn recipe_new() -> crate::Result<()> {
        let recipe_items = vec![RecipeItem::mock()];
        let recipe_photos = vec![RecipePhoto::mock()];
        let directions = String::from("bake it");
        Recipe::new(recipe_items, recipe_photos, directions);
        Ok(())
    }
}
