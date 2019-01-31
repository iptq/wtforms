#[macro_use]
extern crate serde_derive;

pub mod fields;
pub mod html;
pub mod validators;

pub use wtforms_derive::Form;
use serde::{de::DeserializeOwned, Serialize};

pub use crate::fields::Field;
pub use crate::validators::{ValidationError, Validator};

pub trait Form: Serialize + DeserializeOwned {
    type Error: ValidationError;
    fn as_html() -> String;
    fn validate(&self) -> Result<(), Self::Error>;
}
