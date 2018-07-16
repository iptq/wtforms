use failure::Error;

use forms::Form;

pub struct Field<T> {
    inner: T,
}

pub trait FieldExt {
    fn process<'a>(&self, input: &'a str) -> Result<(), Error>;

    fn validate(&self, form: &Form) -> Result<(), Error> {
        Ok(())
    }
}