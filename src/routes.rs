use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use chrono::Utc;
use sqlx::SqlitePool;

use crate::models::{Item, List, ListItem, NewItem, NewList};

pub fn router(db: SqlitePool) -> Router {
    Router::new()
        .route("/lists", get(list_lists))
        .route("/lists", post(add_list))
        .route("/lists/{id}", get(get_list))
        .route("/lists/{id}/finish", post(finish_list))
        .route("/lists/{id}/items", post(set_list_items))
        .route("/items", get(list_items))
        .route("/items", post(add_item))
        .route("/items/{id}", get(get_item))
        .route("/items/{id}", delete(delete_item))
        .with_state(db)
}

async fn list_lists(State(db): State<SqlitePool>) -> Result<Json<Vec<List>>> {
    let lists = sqlx::query!("select * from lists")
        .map(|row| List {
            id: row.list_id,
            name: row.name,
            created: row.created,
            finished: row.finished,
            items: Vec::new(),
        })
        .fetch_all(&db)
        .await?;

    Ok(Json(lists))
}

async fn add_list(State(db): State<SqlitePool>, Json(req): Json<NewList>) -> Result<Json<List>> {
    let now = Utc::now().naive_utc();
    let id = sqlx::query_scalar!(
        r"insert into lists(name, created) values (?, ?) returning list_id",
        req.name,
        now,
    )
    .fetch_one(&db)
    .await?;

    Ok(Json(List {
        id,
        name: req.name,
        created: now,
        finished: None,
        items: Vec::new(),
    }))
}

async fn get_list(State(db): State<SqlitePool>, Path(id): Path<i64>) -> Result<Json<List>> {
    let list = sqlx::query!("select * from lists where list_id = ?", id)
        .fetch_one(&db)
        .await?;

    let items = sqlx::query_as!(
        ListItem,
        r"select
            item_id as id,
            name,
            amount
        from items
        join list_items using (item_id)
        where list_id = ?",
        id
    )
    .fetch_all(&db)
    .await?;

    Ok(Json(List {
        id: list.list_id,
        name: list.name,
        created: list.created,
        finished: list.finished,
        items,
    }))
}

async fn set_list_items(
    State(db): State<SqlitePool>,
    Path(id): Path<i64>,
    Json(req): Json<Vec<ListItem>>,
) -> Result<StatusCode> {
    let mut tr = db.begin().await?;
    sqlx::query!("delete from list_items where list_id = ?", id)
        .execute(&mut *tr)
        .await?;

    for item in req {
        sqlx::query!(
            "insert into list_items values (?, ?, ?)",
            id,
            item.id,
            item.amount
        )
        .execute(&mut *tr)
        .await?;
    }

    tr.commit().await?;
    Ok(StatusCode::OK)
}

async fn finish_list(State(db): State<SqlitePool>, Path(id): Path<i64>) -> Result<StatusCode> {
    let now = Utc::now().naive_utc();
    sqlx::query!("update lists set finished = ? where list_id = ?", now, id)
        .execute(&db)
        .await?;
    Ok(StatusCode::OK)
}

async fn list_items(State(db): State<SqlitePool>) -> Result<Json<Vec<Item>>> {
    let items = sqlx::query_as!(
        Item,
        r"select
            item_id as id,
            name
        from items"
    )
    .fetch_all(&db)
    .await?;

    Ok(Json(items))
}

async fn add_item(State(db): State<SqlitePool>, Json(req): Json<NewItem>) -> Result<Json<Item>> {
    let id = sqlx::query_scalar!(
        r"insert into items(name) values (?) returning item_id",
        req.name
    )
    .fetch_one(&db)
    .await?;

    Ok(Json(Item { id, name: req.name }))
}

async fn get_item(State(db): State<SqlitePool>, Path(id): Path<i64>) -> Result<Json<Item>> {
    let item = sqlx::query_as!(
        Item,
        r"select
            item_id as id,
            name
        from items
        where item_id = ?",
        id
    )
    .fetch_one(&db)
    .await?;

    Ok(Json(item))
}

async fn delete_item(State(db): State<SqlitePool>, Path(id): Path<i64>) -> Result<StatusCode> {
    // NOTE: no cascade on conflict here, since that could mess with active lists
    sqlx::query!(r"delete from items where item_id = ?", id)
        .execute(&db)
        .await?;

    Ok(StatusCode::OK)
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("db error: {0}")]
    Db(#[from] sqlx::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}
