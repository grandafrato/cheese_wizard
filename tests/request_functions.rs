use std::error::Error;

use cheese_wizard::cheese::{
    CheeseData, CheeseRating, CheeseRatingRequest, CheeseRegistry, RegistryCheeseRating,
};
use cheese_wizard::requests::rate_cheese;
use cheese_wizard::user::{UserCheeseRating, UserData};

#[test]
fn adding_cheese_to_a_user() -> Result<(), Box<dyn Error>> {
    let request = r#"
    {
        "rating": 5,
        "cheese": "FooCheeseId"
    }
        "#;

    let mut user = UserData::new();
    let mut cheese_registry = CheeseRegistry::new();
    cheese_registry.insert(CheeseData::default().name("FooCheeseId"))?;

    let cheese_rating_request: CheeseRatingRequest = serde_json::from_str(request)?;

    rate_cheese(cheese_rating_request, &mut user, &mut cheese_registry)?;

    assert!(user
        .cheese_ratings
        .into_iter()
        .any(|x| x == UserCheeseRating("FooCheeseId".to_string(), CheeseRating::new(5).unwrap())));

    assert!(cheese_registry.into_iter().all(|cheese| cheese
        .ratings
        .into_iter()
        .any(|RegistryCheeseRating(user_id, rating)| user_id == user.id
            && rating == CheeseRating::new(5).unwrap())));
    Ok(())
}
