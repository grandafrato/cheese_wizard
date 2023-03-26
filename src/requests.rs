use crate::cheese::{
    CheeseData, CheeseRating, CheeseRegistry, CheeseRegistryError, RegistryCheeseRating,
};
use crate::user::{UserCheeseRating, UserData};
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize)]
pub struct CheeseRatingRequest {
    pub rating: u8,
    pub cheese: String,
}

#[derive(Deserialize)]
pub struct NewCheeseRequest {
    pub name: String,
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

pub fn create_new_cheese(
    request: NewCheeseRequest,
    registry: &mut CheeseRegistry,
) -> Result<(), CheeseRegistryError> {
    registry.insert(CheeseData::default().name(&request.name))?;
    Ok(())
}

pub fn all_cheeses(registry: &CheeseRegistry) -> Vec<CheeseData> {
    let mut cheeses: Vec<CheeseData> = registry.clone().into_iter().collect();
    cheeses.sort();
    cheeses
}
