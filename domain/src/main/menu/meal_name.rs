use common::main::types::base::value_object::ValueObject;
use common::main::types::errors::error::BusinessError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Hash)]
pub struct MealName {
    pub value: String,
}

impl MealName {
    pub fn to_string_value(&self) -> &String {
        &self.value
    }

    pub fn from(name: String) -> Result<Self, CreateMealNameError> {
        if name == *"" {
            Err(CreateMealNameError::EmptyString)
        } else {
            Ok(Self { value: name })
        }
    }
}

impl ValueObject for MealName {}

#[derive(thiserror::Error, Debug)]
pub enum CreateMealNameError {
    #[error("Название еды не может быть пустым")]
    EmptyString,
}

impl BusinessError for CreateMealNameError {}
