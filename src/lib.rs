#[macro_use]
extern crate failure;
#[allow(unused_imports)]
#[macro_use]
extern crate wtforms_derive;

mod fields;
mod forms;

pub use wtforms_derive::*;

pub mod prelude {
    pub use super::fields::{Field, FieldExt};
    pub use super::forms::Form;
}
