
// src/repository/core/entities/template.rs
#[derive(Clone)]
pub struct Template {
    pub name: String,
    pub path: String,
    pub type_: String,
    pub status: String,
}

impl Template {
    pub fn new(name: String, path: String, type_: String, status_value: String) -> Result<Self, String> {
        if name.is_empty() {
            return Err("El nombre no puede estar vacío".to_string());
        }
        if !["type1", "type2"].contains(&type_.as_str()) {
            return Err("Tipo inválido".to_string());
        }
        Ok(Template { name, path, type_ , status: status_value })
    }
}
