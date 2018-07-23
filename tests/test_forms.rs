#[macro_use]
extern crate wtforms;
extern crate failure;

use failure::Error;

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
        #[field]
        password: Field<Password>,
    }
}

#[test]
fn render() {
    #[allow(dead_code)]
    #[derive(Form)]
    struct NameForm {
        #[field(name = "name")]
        name: Field<String>,
    }

    let form = NameForm {
        name: Field::from(String::from("green beans")),
    };
    assert_eq!(
        &form.render().unwrap(),
        r#"<form><input name="green beans" /></form>"#
    );
}
