use common::types::{base::value_object::ValueObject, errors::error::BusinessError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Default, Deserialize, Serialize, Display)]
#[non_exhaustive]
pub struct MealDescription(String);

impl TryFrom<&str> for MealDescription {
    type Error = CreateMealDescriptionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            x if x.is_empty() || x == " " => Err(Self::Error::EmptyDescriptionError),
            _ => Ok(Self(value.to_string())),
        }
    }
}

impl ValueObject for MealDescription {}

#[derive(Debug, PartialEq)]
pub enum CreateMealDescriptionError {
    EmptyDescriptionError,
}

impl BusinessError for CreateMealDescriptionError {}