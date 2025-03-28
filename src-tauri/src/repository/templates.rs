use diesel::prelude::*;
use crate::db::connection::establish_connection;
use crate::models::Template;
use crate::schema::templates::dsl::*;

pub fn get_all_templates(conn: &SqliteConnection) -> QueryResult<Vec<Template>> {
    templates.load::<Template>(conn)
}

pub fn create_template(conn: &SqliteConnection, new_template: &Template) -> QueryResult<usize> {
    diesel::insert_into(templates).values(new_template).execute(conn)
}

pub fn delete_template(conn: &SqliteConnection, template_id: i32) -> QueryResult<usize> {
    diesel::delete(templates.filter(id.eq(template_id))).execute(conn)
}