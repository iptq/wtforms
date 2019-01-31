use crate::fields::Field;

pub trait ValidationError {
    fn old_value(&self) -> String;
}

// TODO: get rid of this
impl ValidationError for () {
    fn old_value(&self) -> String {
        String::new()
    }
}

pub trait Validator<F: Field> {
    fn validate(&self) -> Result<(), ()>;
}

pub struct Length(pub u32, pub u32);

pub struct Required;

pub struct EqualsTo<T>(pub T);
