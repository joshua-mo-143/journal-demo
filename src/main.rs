use axum::{routing::{get, post}, Router, Json, response::IntoResponse as AxumResponse, http::StatusCode, response::Response, extract::{Path, State, Form}};
use serde::{Serialize, Deserialize};
use sqlx::PgPool;
use chrono::{DateTime, Utc};
use askama::Template;
use askama_axum::IntoResponse;

#[derive(Clone)]
pub struct AppState {
    db: PgPool
}

#[derive(sqlx::FromRow, Serialize)]
struct Post {
    title: String,
    body: String,
    date: DateTime<Utc> 
}

#[derive(sqlx::FromRow, Deserialize)]
struct PostSubmit {
    title: String,
    body: String
}

#[derive(Template)]
#[template(path = "post_index.html")]
struct IndexTemplate {
    posts: Vec<Post>
}

#[derive(Template)]
#[template(path = "post_form.html")]
struct PostTemplate {
    post: Post
}

#[derive(Template)]
#[template(path = "post.html")]
struct PostTemplate {
    post: Post
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] db: PgPool,
    ) -> shuttle_axum::ShuttleAxum {
    let state = AppState { db };
    let router = Router::new()
        .route("/", get(get_entries))
        .route("/entries", post(create_entry))
        .route("/entries/create", get(entry_form))
        .route("/entries/:id", get(get_entry_by_id))
        .route("/styles.css", get(styles))
        .with_state(state);

    Ok(router.into())
}

async fn styles() -> impl AxumResponse {
    Response::builder()
        .header("Content-Type", "text/css")
        .body(include_str!("../templates/styles.css").to_owned())
        .unwrap()
}

async fn get_entries(
    State(state): State<AppState>
    ) -> impl IntoResponse {
    let posts = sqlx::query_as::<_, Post>("SELECT title, body, DATE(created_at) FROM entries")
        .fetch_all(&state.db)
        .await
        .unwrap();

    IndexTemplate { posts }

} 

async fn get_entry_by_id(
    State(state): State<AppState>,
    Path(id): Path<i32>
    ) -> impl IntoResponse {
    let post = sqlx::query_as::<_, Post>("SELECT title, body, DATE(created_at) FROM entries WHERE id = $1")
        .bind(id)
        .fetch_one(&state.db)
        .await
        .unwrap();

    PostTemplate { post }
} 

async fn post_entry(
    State(state): State<AppState>,
    Form(post): Form<PostSubmit>
    ) -> StatusCode {
    let query = sqlx::query("INSERT INTO entries (title, body) VALUES ($1, $2)")
        .bind(post.title)
        .bind(post.body)
        .execute(&state.db)
        .await
        .unwrap();

    StatusCode::CREATED 
} 
