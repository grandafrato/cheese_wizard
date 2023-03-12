use std::error::Error;

use cheese_wizard::{rate_cheese, CheeseRating, CheeseRatingRequest, CheeseRegistry, UserData};

#[test]
fn adding_cheese_to_a_user() -> Result<(), Box<dyn Error>> {
    let request = r#"
    {
        rating: 5,
        cheese_id: "FooCheeseId"
    }
        "#;

    let mut user = Userdata {
        name: "Foo",
        ..Default::default()
    };
    let mut cheese_registry = CheeseRegistry::new();

    let cheese_rating_request: CheeseRatingRequest = serde_json::from_str(request)?;

    rate_cheese(cheese_rating_request, &mut user, &mut cheese_registry)?;

    assert!(user
        .cheese_ratings
        .into_iter()
        .all(|x| x == CheeseRating::new("FooCheeseId", 5)));
    todo!("Need to finish designing cheese adding interface.");
}
