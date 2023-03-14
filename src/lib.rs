use serde::Deserialize;
use std::error::Error;
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
    pub cheese_ratings: Vec<UserCheeseRating>,
}

impl UserData {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: "".to_owned(),
            age: 0,
            cheese_ratings: Vec::new(),
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

pub struct CheeseRegistry(Vec<CheeseData>);

impl CheeseRegistry {
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

impl IntoIterator for CheeseRegistry {
    type Item = CheeseData;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(PartialEq, Debug)]
pub struct CheeseRating(u8);

#[derive(Debug, PartialEq)]
pub enum RatingBoundsError {
    ExceedsMaximumRating,
    BelowMinimumRating,
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

#[derive(PartialEq)]
pub struct UserCheeseRating(pub Uuid, pub CheeseRating);

#[derive(PartialEq, Debug)]
pub struct RegistryCheeseRating(pub Uuid, pub CheeseRating);

#[derive(Default)]
pub struct CheeseData {
    name: String,
    pub ratings: Vec<RegistryCheeseRating>,
}

impl CheeseData {
    fn new() -> Self {
        Self::default()
    }

    // Constructors for unit testing
    fn name(self, name: &str) -> Self {
        Self {
            name: name.to_owned(),
            ..self
        }
    }
}

pub fn rate_cheese(
    request: CheeseRatingRequest,
    user: &mut UserData,
    registry: &mut CheeseRegistry,
) -> Result<(), Box<dyn Error>> {
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
        let cheese = CheeseData::new().name("Chedder");

        assert_eq!(cheese.name, "Chedder");
        assert_eq!(cheese.ratings, Vec::new())
    }
}
