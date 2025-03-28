#[derive(Queryable, Insertable)]
#[table_name = "templates"]
pub struct Template {
    pub id: i32,
    pub name: String,
    pub path: String,
    pub type_: String, // `type` es palabra reservada en Rust, así que usa `type_`
    pub created_at: chrono::NaiveDateTime,
    pub modified_at: Option<chrono::NaiveDateTime>, // Puede ser nulo
}