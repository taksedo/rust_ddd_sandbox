use common::types::base::value_object::ValueObject;
use derive_more::Display;
use derive_new::new;
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    new, Debug, Clone, Deserialize, Serialize, PartialEq, Default, Eq, Hash, Copy, Display,
)]
pub struct CustomerId(#[new(value = "Uuid::new_v4()")] Uuid);

impl From<Uuid> for CustomerId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl ValueObject for CustomerId {}