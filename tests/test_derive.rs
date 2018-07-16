#[macro_use]
extern crate wtforms;

use wtforms::prelude::*;

#[test]
fn derive_basic() {
    #[allow(dead_code)]
    #[derive(Form)]
    struct LoginForm {
        username: Field<String>,
        password: Field<String>,
    }
}

#[test]
fn derive_with_attribute() {
    #[allow(dead_code)]
    #[derive(Form)]
    struct LoginForm {
        #[field(name = "username")]
        username: Field<String>,
        #[field(ty = "password")]
        password: Field<String>,
    }
}
