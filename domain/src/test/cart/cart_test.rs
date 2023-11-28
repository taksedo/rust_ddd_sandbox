use std::collections::HashMap;
use std::mem::discriminant;
use std::sync::{Arc, Mutex};

use common::types::main::base::domain_entity::DomainEntityTrait;
use common::types::main::common::count::Count;
use smart_default::SmartDefault;
use time::OffsetDateTime;

use crate::main::cart::cart::Cart;
use crate::main::cart::cart_events::{
    CartCreatedDomainEvent, CartEventEnum, MealAddedToCartDomainEvent,
};
use crate::main::cart::value_objects::cart_id::{CartId, CartIdGenerator};
use crate::test_fixtures::{rnd_cart, rnd_cart_id, rnd_customer_id, rnd_meal};

#[test]
fn create_cart_success() {
    let customer_id = rnd_customer_id();
    let id_generator = Arc::new(Mutex::new(TestCartIdGenerator::default()));
    let mut cart = Cart::create(Arc::clone(&id_generator) as _, customer_id.clone());

    let id = id_generator.lock().unwrap().id;
    assert_eq!(cart.entity_param.id, id);
    assert_eq!(cart.for_customer, customer_id);
    assert!(cart.meals.is_empty());
    assert!(cart.created < OffsetDateTime::now_utc());
    assert!(cart
        .entity_param
        .pop_events()
        .iter()
        .all(|event| discriminant(event)
            == discriminant(&CartCreatedDomainEvent::new(rnd_cart_id()).into())));
}

#[test]
fn add_meal_no_meal_in_cart_success() {
    let mut cart = rnd_cart();
    let meal = rnd_meal();

    cart.add_meal(meal.clone());
    assert!(cart.entity_param.pop_events().iter().all(|event| {
        match event {
            CartEventEnum::MealAddedToCartDomainEvent(_) => true,
            _ => false,
        }
    }));
    assert!(cart.meals.iter().all(|item| {
        let (&item_meal_id, &item_count) = item;
        (item_meal_id == meal.entity_params.id) && (item_count == Count::one())
    }))
}

#[test]
fn add_meal_has_meals_in_cart_success() {
    let meal = rnd_meal();
    let count = Count::try_from(2).unwrap();
    let mut cart = rnd_cart();
    cart.meals.insert(meal.entity_params.id, count);

    cart.add_meal(meal.clone());
    assert!(cart.entity_param.pop_events().iter().all(|event| {
        event
            == &MealAddedToCartDomainEvent::new(cart.entity_param.id, meal.entity_params.id).into()
    }));
    assert!(cart.meals.iter().all(|item| {
        let (&item_meal_id, &item_count) = item;
        (item_meal_id == meal.entity_params.id) && (item_count == Count::try_from(3).unwrap())
    }))
}

#[test]
fn remove_meal_cart_is_empty_success() {
    let meal = rnd_meal();
    let mut cart = rnd_cart();
    cart.remove_meals(meal.entity_params.id);
    assert!(cart.entity_param.pop_events().is_empty());
}

#[test]
fn remove_meal_meal_not_in_cart() {
    let existing_meal = rnd_meal();
    let count = Count::try_from(12).unwrap();
    let non_exixstin_meal = rnd_meal();
    let meals = HashMap::from([(existing_meal.entity_params.id, count)]);

    let mut cart = rnd_cart();
    cart.meals = meals.clone();

    cart.remove_meals(non_exixstin_meal.entity_params.id);
    assert!(cart.entity_param.pop_events().is_empty());
    assert!(cart.meals.iter().all(|item| {
        let (&item_meal_id, &item_count) = item;
        meals.get_key_value(&item_meal_id).unwrap() == (&item_meal_id, &item_count)
    }));
}

#[derive(Debug, SmartDefault)]
struct TestCartIdGenerator {
    #[default(rnd_cart_id())]
    id: CartId,
}

impl CartIdGenerator for TestCartIdGenerator {
    fn generate(&mut self) -> CartId {
        self.id
    }
}
