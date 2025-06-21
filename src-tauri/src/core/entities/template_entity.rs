#[derive(Clone, Debug)]
pub struct Template {
    pub name: String,
    pub file_id: i32,
    pub type_: String,
    pub status: String,
}

impl Template {
    pub fn new(name: String, file_id: i32, type_: String, status: String) -> Result<Self, String> {
        if name.trim().is_empty() {
            return Err("Name cannot be empty".into());
        }
        if !["bug", "nuevo", "mant"].contains(&type_.as_str()) {
            return Err(format!("Invalid type: {}", type_));
        }
        Ok(Self { name, file_id, type_, status })
    }
}