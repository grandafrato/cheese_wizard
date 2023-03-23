use crate::cheese::{CheeseRating, CheeseRatingRequest, CheeseRegistry, RegistryCheeseRating};
use crate::user::{UserCheeseRating, UserData};
use std::error::Error;

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
