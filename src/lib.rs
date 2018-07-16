#[macro_use]
extern crate failure;
#[allow(unused_imports)]
#[macro_use]
extern crate wtforms_derive;

mod forms;
mod fields;

pub use wtforms_derive::*;

pub mod prelude {
    pub use super::forms::Form;
    pub use super::fields::{Field, FieldExt};
}

