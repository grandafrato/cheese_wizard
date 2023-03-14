use std::error::Error;
use uuid::Uuid;

pub struct CheeseRating;
pub struct CheeseRatingRequest;
pub struct CheeseRegistry;
pub struct UserCheeseRating<'a>(pub &'a str, pub CheeseRatingRequest);
pub struct RegistryCheeseRating<'a>(pub &'a str, pub CheeseRatingRequest);

pub struct UserData<'a> {
    id: Uuid,
    name: &'a str,
    age: u8,
}

impl<'a> UserData<'a> {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: "",
            age: 0,
        }
    }

    // Constructors for unit testing
    fn name(self, name: &'a str) -> Self {
        Self { name, ..self }
    }

    fn age(self, age: u8) -> Self {
        Self { age, ..self }
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
}
