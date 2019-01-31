#[macro_use]
extern crate serde_derive;

use wtforms::{
    fields::{Email, Password},
    Form,
};

#[test]
fn basic() {
    #[derive(Form, Serialize, Deserialize)]
    struct RegisterForm {
        pub email: Email,
        #[validators(required, length(5, 10))]
        pub username: String,
        pub password: Password,
        #[validators(equal_to("self.password"))]
        pub confirm_password: Password,
    }

    // generate the html for it
    let html = RegisterForm::as_html();
    panic!("generated: {}", html);

    // pass in some data to validate
    let register_data = RegisterForm {
        email: Email(String::from("test")),
        username: String::from("name"),
        password: Password(String::from("password")),
        confirm_password: Password(String::from("assword")),
    };
    let res = register_data.validate();
    panic!("result: {:?}", res);

    // generate the html again, with the errors
}
