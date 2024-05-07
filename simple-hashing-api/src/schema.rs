use serde::{
    Deserialize,
    Serialize
};

#[derive(Debug, Deserialize, Default)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>
}

#[derive(Debug, Deserialize)]
pub struct ParamOptions {
    pub id: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateUserSchema {
    pub name: String,
    pub email: String,
    pub password: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateUserSchema {
    pub name: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginSchema {
    pub email: String,
    pub password: String
}
