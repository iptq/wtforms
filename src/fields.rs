pub trait Field {}

#[derive(Serialize, Deserialize)]
pub struct Email(pub String);

#[derive(Serialize, Deserialize)]
pub struct Password(pub String);
