use crate::main::menu::meal_name::MealName;
use std::fmt::Debug;

pub trait MealAlreadyExists: Debug {
    fn invoke(&mut self, name: &MealName) -> bool;
}
