use serde::Deserialize;
use std::{collections::HashMap, error::Error, fmt::Display};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CheeseRatingRequest {
    rating: u8,
    cheese: String,
}

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

#[derive(Clone, Debug, PartialEq)]
pub struct CheeseRegistry(HashMap<String, CheeseData>);

#[derive(Debug, PartialEq)]
pub enum CheeseRegistryError {
    DuplicateCheeseName,
    NoSuchCheeseInRegistry,
}

impl Error for CheeseRegistryError {}
impl Display for CheeseRegistryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoSuchCheeseInRegistry => write!(f, "No such cheese in the registry"),
            Self::DuplicateCheeseName => write!(
                f,
                "Cannot insert cheese, cheese names must be unique across a registry."
            ),
        }
    }
}

impl CheeseRegistry {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn insert(&mut self, cheese: CheeseData) -> Result<(), CheeseRegistryError> {
        if self.0.contains_key(&cheese.name) {
            Err(CheeseRegistryError::DuplicateCheeseName)
        } else {
            self.0.insert(cheese.name.clone(), cheese);
            Ok(())
        }
    }

    pub fn get_mut(&mut self, cheese_name: &str) -> Result<&mut CheeseData, CheeseRegistryError> {
        if let Some(cheese) = self.0.get_mut(cheese_name) {
            Ok(cheese)
        } else {
            Err(CheeseRegistryError::NoSuchCheeseInRegistry)
        }
    }
}

impl IntoIterator for CheeseRegistry {
    type Item = CheeseData;
    type IntoIter = std::collections::hash_map::IntoValues<String, Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_values()
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct CheeseRating(u8);

#[derive(Debug, PartialEq)]
pub enum RatingBoundsError {
    ExceedsMaximumRating,
    BelowMinimumRating,
}

impl Error for RatingBoundsError {}
impl Display for RatingBoundsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ExceedsMaximumRating => {
                write!(f, "The given rating exceeds the maximum rating of 10")
            }
            Self::BelowMinimumRating => {
                write!(f, "The given rating is beloew the minimum rating of 1")
            }
        }
    }
}

impl CheeseRating {
    pub fn new(rating: u8) -> Result<Self, RatingBoundsError> {
        if rating < 1 {
            Err(RatingBoundsError::BelowMinimumRating)
        } else if rating > 10 {
            Err(RatingBoundsError::ExceedsMaximumRating)
        } else {
            Ok(Self(rating))
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct RegistryCheeseRating(pub Uuid, pub CheeseRating);

#[derive(Default, PartialEq, Debug, Clone)]
pub struct RegistryCheeseRatingMap(HashMap<Uuid, CheeseRating>);

impl RegistryCheeseRatingMap {
    fn insert(&mut self, RegistryCheeseRating(user_id, rating): RegistryCheeseRating) {
        self.0.insert(user_id, rating);
    }

    fn get(&self, uuid: Uuid) -> CheeseRating {
        self.0[&uuid]
    }
}

impl IntoIterator for RegistryCheeseRatingMap {
    type Item = RegistryCheeseRating;
    type IntoIter = std::iter::Map<
        std::collections::hash_map::IntoIter<Uuid, CheeseRating>,
        fn((Uuid, CheeseRating)) -> Self::Item,
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.0
            .into_iter()
            .map(|(user_id, cheese_rating)| RegistryCheeseRating(user_id, cheese_rating))
    }
}

#[derive(Default, PartialEq, Debug, Clone)]
pub struct CheeseData {
    name: String,
    pub ratings: RegistryCheeseRatingMap,
}

impl CheeseData {
    // Constructors for unit testing
    pub fn name(self, name: &str) -> Self {
        Self {
            name: name.to_owned(),
            ..self
        }
    }

    fn insert_rating(&mut self, rating: RegistryCheeseRating) {
        self.ratings.insert(rating);
    }
}

pub fn rate_cheese(
    request: CheeseRatingRequest,
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
    fn cheese_ratings_must_be_within_bounds() {
        for rating_number in 1..=10 {
            assert_eq!(
                CheeseRating(rating_number),
                CheeseRating::new(rating_number).unwrap()
            );
        }

        assert_eq!(
            Err(RatingBoundsError::ExceedsMaximumRating),
            CheeseRating::new(11)
        );
        assert_eq!(
            Err(RatingBoundsError::BelowMinimumRating),
            CheeseRating::new(0)
        );
    }

    #[test]
    fn new_cheese_data() {
        let cheese = CheeseData::default().name("Chedder");

        assert_eq!(cheese.name, "Chedder");
        assert_eq!(cheese.ratings, RegistryCheeseRatingMap::default())
    }

    #[test]
    fn registered_cheese_ratings_accepts_a_registered_cheese_rating() {
        let mut registered_cheese_ratings = RegistryCheeseRatingMap::default();
        let user_id = Uuid::new_v4();
        registered_cheese_ratings
            .insert(RegistryCheeseRating(user_id, CheeseRating::new(5).unwrap()));

        let cheese_ratings_vec: Vec<RegistryCheeseRating> =
            registered_cheese_ratings.into_iter().collect();
        assert_eq!(
            cheese_ratings_vec[0],
            RegistryCheeseRating(user_id, CheeseRating::new(5).unwrap())
        )
    }

    #[test]
    fn inserting_a_rating_into_cheese_data_adds_a_rating() {
        let mut cheese = CheeseData::default();
        let user_id = Uuid::new_v4();
        let rating = RegistryCheeseRating(user_id, CheeseRating::new(5).unwrap());

        cheese.insert_rating(rating);

        assert_eq!(cheese.ratings.get(user_id), CheeseRating::new(5).unwrap());
    }

    #[test]
    fn insert_cheese_into_registry() -> Result<(), CheeseRegistryError> {
        let cheese = CheeseData::default();
        let mut registry = CheeseRegistry::new();

        registry.insert(cheese.clone())?;

        assert_eq!(registry.into_iter().next().unwrap(), cheese);
        Ok(())
    }

    #[test]
    fn cheese_names_are_unique_across_a_registry() -> Result<(), CheeseRegistryError> {
        let cheese_1 = CheeseData::default().name("Foo");
        let cheese_2 = CheeseData::default().name("Bar");

        let mut registry = CheeseRegistry::new();

        registry.insert(cheese_1.clone())?;
        registry.insert(cheese_2.clone())?;

        // The registry accepts and holds working inputs.
        let registry_vec: Vec<CheeseData> = registry.clone().into_iter().collect();
        assert!(registry_vec.contains(&cheese_1));
        assert!(registry_vec.contains(&cheese_2));

        assert_eq!(
            Err(CheeseRegistryError::DuplicateCheeseName),
            registry.insert(cheese_1)
        );

        Ok(())
    }

    #[test]
    fn get_mut_cheese_from_registry() -> Result<(), CheeseRegistryError> {
        let cheese = CheeseData::default().name("Chedder");
        let mut registry = CheeseRegistry::new();

        registry.insert(cheese.clone())?;

        assert_eq!(registry.clone().into_iter().next().unwrap(), cheese);

        let registry_clone = registry.clone();
        let cheese_mut = registry.get_mut("Chedder")?;

        assert_eq!(*cheese_mut, cheese);
        cheese_mut.insert_rating(RegistryCheeseRating(
            Uuid::new_v4(),
            CheeseRating::new(5).unwrap(),
        ));

        assert_ne!(registry_clone, registry);
        Ok(())
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
