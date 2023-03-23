pub mod cheese;
pub mod user;

use cheese::{CheeseRating, CheeseRegistry, RegistryCheeseRating};
use std::error::Error;
use user::{UserCheeseRating, UserData};

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
