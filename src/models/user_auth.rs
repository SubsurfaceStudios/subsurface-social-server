use diesel::prelude::*;

#[derive(Queryable, Debug, Selectable)]
#[diesel(table_name = crate::schema::authentication)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserAuth {
    pub id: uuid::Uuid,
    pub username: String,
    pub salt: String,
    pub hash: Vec<u8>,
    pub created: chrono::NaiveDateTime,
    pub updated: chrono::NaiveDateTime,
}