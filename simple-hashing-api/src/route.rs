use axum::{
    routing::{delete, get, post, put},
    Router
};

use std::sync::Arc;

use crate::{
    handler::{
        health_check_handler,
        get_all_users,
        get_user_by_id,
        create_user,
        update_user,
        delete_user,
        login
    },
    AppState
};

pub fn create_router(
    app_state: Arc<AppState>
) -> Router {
    return Router::new()
        .route("/api/v1.0/healthcheck", get(health_check_handler))
        .route("/api/v1.0/get-all-users", get(get_all_users))
        .route("/api/v1.0/:id", get(get_user_by_id))
        .route("/api/v1.0/create-user", post(create_user))
        .route("/api/v1.0/update-user/:id", put(update_user))
        .route("/api/v1.0/delete-user/:id", delete(delete_user))
        .route("/api/v1.0/login", post(login)
    ).with_state(app_state);
}
