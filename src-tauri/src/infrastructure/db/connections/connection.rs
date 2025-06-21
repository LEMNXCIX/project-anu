use diesel::Connection;

pub trait ConnectionProvider {
    type Connection: Connection;
    fn get_connection(&self) -> Result<Self::Connection, String>;
}