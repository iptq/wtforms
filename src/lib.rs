#[macro_use]
extern crate serde_derive;
pub extern crate frunk as _frunk;

pub mod fields;
pub mod html;
pub mod validators;

use serde::{de::DeserializeOwned, Serialize};
pub use wtforms_derive::Form;

pub use crate::fields::Field;
pub use crate::validators::{ValidationError, Validator};

pub trait Form: Serialize + DeserializeOwned {
    type Error: ValidationError;
    fn as_html() -> String;
    fn validate(self) -> Result<Self, Self::Error>;
}
