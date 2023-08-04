use diesel::prelude::*;

#[derive(Queryable, Debug, Selectable)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    pub id: uuid::Uuid,
    pub title: Option<String>,
    pub body: String,
    pub author: uuid::Uuid,
    pub created: chrono::NaiveDateTime,
    pub updated: chrono::NaiveDateTime,
    pub deleted: Option<chrono::NaiveDateTime>,
    #[sql_type = "Jsonb"]
    pub attachments: serde_json::Value,
}