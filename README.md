wtforms
=======

**NOTE** it doesn't work yet

As the name implies, it's heavily inspired by the Python library [wtforms](https://wtforms.readthedocs.io).

Usage
-----

```rs
#[derive(Form, Serialize, Deserialize)]
struct RegisterForm {
    pub email: Email,
    #[validators(required, length(5, 10))]
    pub username: String,
    pub password: Password,
    #[validators(equal_to("self.password"))]
    pub confirm_password: Password,
}
```

Then you can `RegisterForm::as_html()` to generate HTML code for it.

You can also validate a `RegisterForm` that's been deserialized from, for example `Form<RegisterForm>` in some web framework.

Caveats
-------

Customization not supported yet.

Contact
-------

Author: Michael Zhang

License: MIT
