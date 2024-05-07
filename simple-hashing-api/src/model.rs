use serde::{
    Deserialize,
    Serialize
};

use sqlx::FromRow;
use chrono::DateTime;
use chrono::Utc;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct UserModel {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserModelResponse {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>
}

pub fn model_to_response(
    user_model: &UserModel
) -> UserModelResponse {
    return UserModelResponse {
        id: user_model.id.to_owned(),
        name: user_model.name.to_owned(),
        email: user_model.email.to_owned(),
        password: user_model.password.to_owned(),
        created_at: user_model.created_at.to_owned(),
        updated_at: user_model.updated_at.to_owned()
    }
}
