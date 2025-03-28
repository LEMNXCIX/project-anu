use diesel::prelude::*;
use crate::db::connection::establish_connection;
use crate::models::Project;
use crate::schema::projects::dsl::*;

pub fn get_all_projects(conn: &SqliteConnection) -> QueryResult<Vec<Project>> {
    projects.load::<Project>(conn)
}

pub fn create_project(conn: &SqliteConnection, new_project: &Project) -> QueryResult<usize> {
    diesel::insert_into(projects).values(new_project).execute(conn)
}

pub fn delete_project(conn: &SqliteConnection, project_id: i32) -> QueryResult<usize> {
    diesel::delete(projects.filter(id.eq(project_id))).execute(conn)
}