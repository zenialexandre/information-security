use axum::{
    extract::{
        Path,
        Query,
        State
    },
    http::StatusCode,
    response::IntoResponse,
    Json
};

use serde_json::{
    json,
    Value
};

use sqlx::{
    mysql::MySqlQueryResult,
    Error
};

use std::sync::Arc;
use uuid::Uuid;
use pwhash::bcrypt;

use crate::{
    model::{
        model_to_response,
        UserModel,
        UserModelResponse
    },
    schema::{
        CreateUserSchema,
        UpdateUserSchema,
        FilterOptions,
        LoginSchema
    },
    AppState
};

pub async fn health_check_handler() -> impl IntoResponse {
    let json_response = serde_json::json!({
        "status": "ok",
        "message": "API Services"
    });
    return Json(json_response);
}

pub async fn get_all_users(
    options: Option<Query<FilterOptions>>,
    State(data): State<Arc<AppState>>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let Query(options) = options.unwrap_or_default();
    let limit: usize = options.limit.unwrap_or(10);
    let offset: usize = (options.page.unwrap_or(1) - 1) * limit;

    let users: Vec<UserModel> = sqlx::query_as!(
        UserModel,
        r#"select * from user order by id limit ? offset ?"#,
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.database)
    .await
    .map_err(|error: Error| {
        let error_response = serde_json::json!({
            "status": "error",
            "message": format!("Database error: {}", error)
        });
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response));
    })?;

    let user_responses: Vec<UserModelResponse> = users.iter()
        .map(|user| model_to_response(&user))
        .collect::<Vec<UserModelResponse>>();

    let json_response: Value = serde_json::json!({
        "status": "ok",
        "count": user_responses.len(),
        "users": user_responses
    });
    return Ok(Json(json_response));
}

pub async fn get_user_by_id(
    Path(id): Path<Uuid>,
    State(data): State<Arc<AppState>>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user_by_id: Result<UserModel, Error> = sqlx::query_as!(
        UserModel,
        r#"select * from user where id = ?"#,
        id.to_string()
    )
    .fetch_one(&data.database)
    .await;

    match user_by_id {
        Ok(user) => {
            let user_response: Value = serde_json::json!({
                "status": "success",
                "data": serde_json::json!({
                    "user": model_to_response(&user)
                })
            });

            return Ok((StatusCode::OK, Json(user_response)));
        },
        Err(Error::RowNotFound) => {
            let row_not_found_response: Value = serde_json::json!({
                "status": "fail",
                "message": format!("User with ID: {} not found.", id)
            });

            return Err((StatusCode::NOT_FOUND, Json(row_not_found_response)));
        },
        Err(error) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": format!("{:?}", error)
                }))
            ));
        }
    };
}

pub async fn create_user(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateUserSchema>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id: String = Uuid::new_v4().to_string();
    let create_user_query = sqlx::query(
        r#"insert into user (id, name, email, password) values (?, ?, ?, ?)"#
    )
    .bind(id.clone())
    .bind(body.name.to_string())
    .bind(body.email.to_string())
    .bind(bcrypt::hash(body.password.to_string()).unwrap())
    .execute(&data.database)
    .await
    .map_err(|error: Error| error.to_string());

    if let Err(error) = create_user_query {
        if error.contains("Duplicated entry") {
            let error_response = serde_json::json!({
                "status": "error",
                "message": "User already exists."
            });
            return Err((StatusCode::CONFLICT, Json(error_response)));
        }

        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "status": error,
                "message": format!("{:?}", error)
            }))
        ));
    }

    let user_model: UserModel = sqlx::query_as!(UserModel, r#"select * from user where id = ?"#, id)
        .fetch_one(&data.database)
        .await
        .map_err(|error: Error| {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": format!("{:?}", error)
                }))
            );
        })?;

    let user_model_response: Value = serde_json::json!({
        "status": "success",
        "data": serde_json::json!({
            "user": model_to_response(&user_model)
        })
    });
    return Ok((StatusCode::OK, Json(user_model_response)));
}

pub async fn update_user(
    Path(id): Path<Uuid>,
    State(data): State<Arc<AppState>>,
    Json(body): Json<UpdateUserSchema>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let update_result: MySqlQueryResult = sqlx::query!(
        r#"update user set name = ? where id = ?"#,
        body.name.to_owned(),
        id.to_string()
    )
    .execute(&data.database)
    .await
    .map_err(|error: Error| {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "status": "error",
                "message": format!("{:?}", error)
            }))
        )
    })?;

    if update_result.rows_affected() == 0 {
        let no_rows_affected_response: Value = serde_json::json!({
            "status": "fail",
            "message": format!("User with ID: {} not found.", id)
        });
        return Err((StatusCode::NOT_FOUND, Json(no_rows_affected_response)));
    }
    return Ok(StatusCode::OK);
}

pub async fn delete_user(
    Path(id): Path<Uuid>,
    State(data): State<Arc<AppState>>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let delete_user_query: MySqlQueryResult = sqlx::query!(
        r#"delete from user where id = ?"#, id.to_string()
    )
    .execute(&data.database)
    .await
    .map_err(|error: Error| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "status": "error",
                "message": format!("{:?}", error)
            }))
        )
    })?;

    if delete_user_query.rows_affected() == 0 {
        let error_response = serde_json::json!({
            "status": "error",
            "message": format!("User with ID: {} not found.", id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }
    return Ok(StatusCode::OK);
}

pub async fn login(
    State(data): State<Arc<AppState>>,
    Json(body): Json<LoginSchema>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user_by_email: UserModel = sqlx::query_as!(
        UserModel,
        r#"select * from user where email = ?"#,
        body.email.to_string()
    )
    .fetch_one(&data.database)
    .await
    .map_err(|error: Error| {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "status": "error",
                "message": format!("Error: {:?}", error) 
            }))
        );
    })?;

    let is_password_valid: bool =
        bcrypt::verify(body.password.to_string(), user_by_email.password.as_str());
    let valid_password_response: Value = serde_json::json!({
        "status": "ok",
        "message": format!("Password {} correct for this user! You are now logged in!", body.password.to_string())
    });
    let invalid_password_response: Value = serde_json::json!({
        "status": "error",
        "message": format!("Password {} isn't correct for this user.", body.password.to_string())
    });

    if is_password_valid {
        return Ok((StatusCode::OK, Json(valid_password_response)));
    }
    return Ok((StatusCode::NOT_ACCEPTABLE, Json(invalid_password_response)));
}
