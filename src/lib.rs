pub mod cheese;

use cheese::{CheeseRating, CheeseRegistry, RegistryCheeseRating};
use std::{collections::HashMap, error::Error, fmt::Display};
use uuid::Uuid;

pub struct UserData {
    pub id: Uuid,
    name: String,
    age: u8,
    pub cheese_ratings: UserCheeseRatingMap,
}

#[derive(Debug, PartialEq)]
pub enum UserDataError {
    DuplicateCheeseName,
}

impl Error for UserDataError {}
impl Display for UserDataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DuplicateCheeseName => write!(
                f,
                "Cannot insert cheese, cheese names must be unique across the cheese_ratings."
            ),
        }
    }
}

impl UserData {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: "".to_owned(),
            age: 0,
            cheese_ratings: UserCheeseRatingMap::default(),
        }
    }

    fn insert_rating(&mut self, user_rating: UserCheeseRating) -> Result<(), UserDataError> {
        if self.cheese_ratings.0.contains_key(&user_rating.0) {
            Err(UserDataError::DuplicateCheeseName)
        } else {
            self.cheese_ratings.insert(user_rating);
            Ok(())
        }
    }

    // Constructors for unit testing
    fn name(self, name: &str) -> Self {
        Self {
            name: name.to_owned(),
            ..self
        }
    }

    fn age(self, age: u8) -> Self {
        Self { age, ..self }
    }
}

#[derive(PartialEq, Clone)]
pub struct UserCheeseRating(pub String, pub CheeseRating);

#[derive(Default, PartialEq, Debug, Clone)]
pub struct UserCheeseRatingMap(HashMap<String, CheeseRating>);

impl UserCheeseRatingMap {
    fn insert(&mut self, UserCheeseRating(user_id, rating): UserCheeseRating) {
        self.0.insert(user_id, rating);
    }

    fn get(&self, name: String) -> CheeseRating {
        self.0[&name]
    }
}
impl IntoIterator for UserCheeseRatingMap {
    type Item = UserCheeseRating;
    type IntoIter = std::iter::Map<
        std::collections::hash_map::IntoIter<String, CheeseRating>,
        fn((String, CheeseRating)) -> Self::Item,
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.0
            .into_iter()
            .map(|(cheese_name, cheese_rating)| UserCheeseRating(cheese_name, cheese_rating))
    }
}

pub fn rate_cheese(
    request: cheese::CheeseRatingRequest,
    user: &mut UserData,
    registry: &mut CheeseRegistry,
) -> Result<(), Box<dyn Error>> {
    let cheese = registry.get_mut(&request.cheese)?;
    let rating = CheeseRating::new(request.rating)?;
    cheese.insert_rating(RegistryCheeseRating(user.id, rating));
    user.insert_rating(UserCheeseRating(request.cheese, rating))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_user_data() {
        let user = UserData::new().name("Jeffery Hugo").age(18);

        assert_eq!(user.name, "Jeffery Hugo");
        assert_eq!(user.age, 18);
        assert!(!user.id.is_nil());
    }

    #[test]
    fn new_user_has_unique_id() {
        let user_1 = UserData::new();
        let user_2 = UserData::new();

        assert_ne!(user_1.id, user_2.id);
    }

    #[test]
    fn inserting_a_rating_into_user_data_adds_a_rating() -> Result<(), UserDataError> {
        let mut user = UserData::new();
        let cheese_name = "Chedder".to_owned();
        let rating = UserCheeseRating(cheese_name.clone(), CheeseRating::new(5).unwrap());

        user.insert_rating(rating)?;

        assert_eq!(
            user.cheese_ratings.get(cheese_name),
            CheeseRating::new(5).unwrap()
        );

        Ok(())
    }

    #[test]
    fn cheese_names_are_unique_across_user_data_ratings() -> Result<(), UserDataError> {
        let cheese_1_rating = UserCheeseRating("Foo".to_owned(), CheeseRating::new(5).unwrap());
        let cheese_2_rating = UserCheeseRating("Bar".to_owned(), CheeseRating::new(5).unwrap());

        let mut user = UserData::new();

        user.insert_rating(cheese_1_rating.clone())?;
        user.insert_rating(cheese_2_rating.clone())?;

        // The registry accepts and holds working inputs.
        let registry_vec: Vec<UserCheeseRating> = user.cheese_ratings.clone().into_iter().collect();
        assert!(registry_vec.contains(&cheese_1_rating));
        assert!(registry_vec.contains(&cheese_2_rating));

        assert_eq!(
            Err(UserDataError::DuplicateCheeseName),
            user.insert_rating(cheese_1_rating)
        );

        Ok(())
    }
}
