use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct NewItem {
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct Item {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct NewList {
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct List {
    pub id: i64,
    pub name: String,
    pub created: NaiveDateTime,
    pub finished: Option<NaiveDateTime>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ListItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListItem {
    pub id: i64,
    #[serde(default)]
    pub name: String,
    pub amount: i64,
}
