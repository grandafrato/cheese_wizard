use std::error::Error;

use cheese_wizard::cheese::{
    CheeseData, CheeseRating, CheeseRegistry, RegistryCheeseRating, RegistryCheeseRatingMap,
};
use cheese_wizard::requests::{self, CheeseRatingRequest, NewCheeseRequest};
use cheese_wizard::user::{UserCheeseRating, UserData};

#[test]
fn cheese_rating_request() -> Result<(), Box<dyn Error>> {
    let json_request = r#"
    {
        "rating": 5,
        "cheese": "FooCheeseId"
    }
        "#;

    let mut user = UserData::new();
    let mut cheese_registry = CheeseRegistry::new();
    cheese_registry.insert(CheeseData::default().name("FooCheeseId"))?;

    let cheese_rating_request: CheeseRatingRequest = serde_json::from_str(json_request)?;

    requests::rate_cheese(cheese_rating_request, &mut user, &mut cheese_registry)?;

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

#[test]
fn new_cheese_request() -> Result<(), Box<dyn Error>> {
    let json_request = r#"
    {
        "name": "Chedder"
    }
        "#;

    let mut cheese_registry = CheeseRegistry::new();

    let new_cheese_request: NewCheeseRequest = serde_json::from_str(json_request)?;

    requests::create_new_cheese(new_cheese_request, &mut cheese_registry)?;

    assert!(cheese_registry
        .into_iter()
        .any(|cheese| cheese.name == "Chedder"));
    Ok(())
}

#[test]
fn all_cheeses_request() -> Result<(), Box<dyn Error>> {
    let mut cheese_registry = CheeseRegistry::new();

    let cheese_names = ["Chedder", "Mozzerella", "Colby Jack"]
        .into_iter()
        .map(|name| {
            serde_json::from_str::<NewCheeseRequest>(&format!("{{\"name\": \"{}\"}}", name))
                .unwrap()
        });

    for cheese_request in cheese_names.clone() {
        requests::create_new_cheese(cheese_request, &mut cheese_registry)?;
    }

    let mut cheeses = cheese_names
        .map(|req| CheeseData {
            name: req.name,
            ratings: RegistryCheeseRatingMap::default(),
        })
        .collect::<Vec<CheeseData>>();
    cheeses.sort();

    assert_eq!(requests::all_cheeses(&cheese_registry), cheeses);

    Ok(())
}
