use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;
use uuid::Uuid;

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
    pub name: String,
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

    pub fn insert_rating(&mut self, rating: RegistryCheeseRating) {
        self.ratings.insert(rating);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
}
