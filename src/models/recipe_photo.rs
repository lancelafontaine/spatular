#[derive(Debug)]
pub struct RecipePhoto {
    // TODO: make this a URL struct
    url: String,
}

impl RecipePhoto {
    fn new(url: String) -> Self {
        Self { url }
    }
}
#[cfg(test)]
mod mocks {
    use super::*;

    impl RecipePhoto {
        pub fn mock() -> Self {
            Self::new(String::from("https://example.com"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recipe_photo_mock() -> crate::Result<()> {
        RecipePhoto::mock();
        Ok(())
    }

    #[test]
    fn recipe_photo_new() -> crate::Result<()> {
        let url = String::from("https://foo.bar");
        RecipePhoto::new(url);
        Ok(())
    }
}
