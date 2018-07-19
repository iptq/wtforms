use failure::Error;

use Field;

pub trait Render {
    fn render(&self) -> Result<String, Error>;
}

impl Render for Field<String> {
    fn render(&self) -> Result<String, Error> {
        Ok(format!("<input {} />", ""))
    }
}
