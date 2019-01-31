use std::fmt::{self, Write};

use crate::fields;
use crate::validators::ValidationError;

pub trait AsHtml {
    fn as_html<T: ValidationError>(
        writer: &mut impl Write,
        name: &str,
        err: Option<T>,
    ) -> fmt::Result;
}

impl AsHtml for String {
    fn as_html<T: ValidationError>(w: &mut impl Write, name: &str, err: Option<T>) -> fmt::Result {
        write!(w, "<input type='text' name='{}'", name)?;
        if let Some(err) = err {
            write!(w, " value='{}'", err.old_value())?;
        }
        write!(w, " />")
    }
}

impl AsHtml for fields::Password {
    fn as_html<T: ValidationError>(w: &mut impl Write, name: &str, err: Option<T>) -> fmt::Result {
        write!(w, "<input type='password' name='{}'", name)?;
        if let Some(err) = err {
            write!(w, " value='{}'", err.old_value())?;
        }
        write!(w, " />")
    }
}

impl AsHtml for fields::Email {
    fn as_html<T: ValidationError>(w: &mut impl Write, name: &str, err: Option<T>) -> fmt::Result {
        write!(w, "<input type='email' name='{}'", name)?;
        if let Some(err) = err {
            write!(w, " value='{}'", err.old_value())?;
        }
        write!(w, " />")
    }
}
