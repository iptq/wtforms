#[macro_use]
extern crate wtforms;

use wtforms::*;

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
        #[field(name = "username", autofocus)]
        username: Field<String>,
        #[field(ty = "password")]
        password: Field<String>,
    }
}
