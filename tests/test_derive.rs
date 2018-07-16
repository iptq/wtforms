#[macro_use]
extern crate wtforms;

use wtforms::prelude::*;

#[test]
fn derive_basic() {
    #[derive(Form)]
    struct LoginForm {
        username: Field<String>,
        password: Field<String>,
    }
}

#[test]
fn derive_with_attribute() {
    #[derive(Form)]
    struct LoginForm {
        #[form(name = "username")]
        username: Field<String>,
        #[form(ty = "password")]
        password: Field<String>,
    }
}
