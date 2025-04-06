pub trait ConnectionProvider {
    type Connection; // Add LoadConnection
    fn get_connection(&mut self) -> Result<&mut Self::Connection, String>; // Ahora mutable
}
