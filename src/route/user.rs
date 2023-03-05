use axum::{
    extract::{path, Path, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{Acquire, Pool, Row, Sqlite};

use crate::connection;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    id: i64,
    name: String,
    email: String,
}

impl IntoResponse for User {
    fn into_response(self) -> axum::response::Response {
        let body = serde_json::to_string(&self).unwrap();
        (StatusCode::OK, body).into_response()
    }
}

impl User {
    fn new(id: i64, name: String, email: String) -> Self {
        User { id, name, email }
    }

    fn json_response(&self) -> Response {
        let body = serde_json::to_string(&self).unwrap();
        (StatusCode::OK, body).into_response()
    }

    fn yaml_response(&self) -> Response {
        let body = serde_yaml::to_string(&self).unwrap();
        (StatusCode::OK, body).into_response()
    }

    async fn get_all(pool: &Pool<Sqlite>) -> Vec<User> {
        sqlx::query_as!(User, r#"SELECT * FROM User"#)
            .fetch_all(pool)
            .await
            .unwrap()
    }

    async fn get_all_json(pool: &Pool<Sqlite>) -> Response {
        let body = serde_json::to_string(&User::get_all(pool).await).unwrap();
        (StatusCode::OK, body).into_response()
    }

    async fn get_all_yaml(pool: &Pool<Sqlite>) -> Response {
        let body = serde_yaml::to_string(&User::get_all(pool).await).unwrap();
        (StatusCode::OK, body).into_response()
    }
}

pub async fn route() -> Router {
    let pool = connection::db_connection().await;
    Router::new()
        .route("/", get(get_user))
        .route("/:id", get(get_user_id))
        .with_state(pool)
}

async fn get_user(headers: HeaderMap, State(pool): State<Pool<Sqlite>>) -> Response {
    let x = crate::encoding::AccetedEncoding::from_header(headers);

    let xxx = User::get_all(&pool).await;
    println!("{:?}", xxx);

    if x.is_none() {
        return (StatusCode::BAD_REQUEST, "Bad Request").into_response();
    }

    let ae = x.unwrap();

    return match ae {
        crate::encoding::AccetedEncoding::Json => User::get_all_json(&pool).await,
        crate::encoding::AccetedEncoding::Yaml => User::get_all_yaml(&pool).await,
        _ => (StatusCode::BAD_REQUEST, "Bad Request").into_response(),
    };
}

async fn get_user_id(
    Path(id): Path<i32>,
    State(pool): State<Pool<Sqlite>>,
    headers: HeaderMap,
) -> Response {
    let x = crate::encoding::AccetedEncoding::from_header(headers);

    if x.is_none() {
        return (StatusCode::BAD_REQUEST, "Bad Request").into_response();
    }

    let ae = x.unwrap();

    let user = sqlx::query_as!(User, r#"SELECT * FROM User WHERE id = ?"#, id)
        .fetch_one(&pool)
        .await;

    if user.is_err() {
        return (StatusCode::NOT_FOUND, "Not Found").into_response();
    }

    let user = user.unwrap();

    return match ae {
        crate::encoding::AccetedEncoding::Json => user.json_response(),
        crate::encoding::AccetedEncoding::Yaml => user.yaml_response(),
        _ => (StatusCode::BAD_REQUEST, "Bad Request").into_response(),
    };
}
