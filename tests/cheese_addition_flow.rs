use std::error::Error;

use cheese_wizard::{
    rate_cheese, CheeseRating, CheeseRatingRequest, CheeseRegistry, RegistryCheeseRating,
    UserCheeseRating, UserData,
};

#[test]
fn adding_cheese_to_a_user() -> Result<(), Box<dyn Error>> {
    let request = r#"
    {
        rating: 5,
        cheese_id: "FooCheeseId"
    }
        "#;

    let mut user = UserData::new();
    let mut cheese_registry = CheeseRegistry::new();

    let cheese_rating_request: CheeseRatingRequest = serde_json::from_str(request)?;

    rate_cheese(cheese_rating_request, &mut user, &mut cheese_registry)?;

    assert!(user
        .cheese_ratings
        .into_iter()
        .all(|x| x == UserCheeseRating("FooCheeseId".into(), CheeseRating::new(5))));

    assert!(cheese_registry.into_iter().all(|cheese| cheese
        .into_iter()
        .all(|RegistryCheeseRating(user_id, rating)| user_id == user.id
            && rating == CheeseRating::new(5))));
    todo!("Need to finish designing cheese adding interface.");
}
