use crate::shared::response::ApiResponse;
use log::error;
use serde_json::{from_str, to_string_pretty, Value};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;

pub trait ConfigProvider {
    fn get_item(&self, key: String) -> ApiResponse;
}

pub struct RealConfigProvider;

impl ConfigProvider for RealConfigProvider {
    /// Obtiene un item específico del archivo JSON.
    ///
    /// # Argumentos
    /// * `key` - Clave del item a obtener.
    ///
    /// # Retorno
    /// Retorna una `ApiResponse` con el valor solicitado o un error.
    ///
    /// # Errores
    /// Falla si el archivo no existe, no se puede leer o la clave no existe.
    fn get_item(&self, key: String) -> ApiResponse {
        let path = get_json_path();
        if !path.exists() {
            return ApiResponse::new_error("Archivo no encontrado".to_string(), vec![]);
        }

        match read_json_file(&path) {
            Ok(json) => {
                if let Value::Object(map) = json {
                    match map.get(&key) {
                        Some(value) => {
                            ApiResponse::new_success(value.clone(), "Item encontrado".to_string())
                        }
                        None => ApiResponse::new_error("Clave no encontrada".to_string(), vec![]),
                    }
                } else {
                    ApiResponse::new_error("El JSON raíz debe ser un objeto".to_string(), vec![])
                }
            }
            Err(response) => response,
        }
    }
}

// Variable global para el proveedor (usada en tests)
pub static mut CONFIG_PROVIDER: Option<&dyn ConfigProvider> = None;

pub fn get_item(key: String) -> ApiResponse {
    unsafe { CONFIG_PROVIDER.unwrap_or(&RealConfigProvider).get_item(key) }
}
/// Obtiene la ruta del archivo JSON en el directorio de datos de la aplicación.
///
/// # Retorno
/// Retorna una `PathBuf` apuntando al archivo `database.json` en el directorio de la app.
fn get_json_path() -> PathBuf {
    let config_path = dirs::config_local_dir().unwrap();
    return config_path.join("project-anu").join("config.json");
}

/// Crea un archivo JSON inicial con los datos proporcionados.
///
/// # Argumentos
/// * `json_data` - Datos JSON iniciales a escribir.
///
/// # Retorno
/// Retorna una `ApiResponse` con el resultado de la operación.
///
/// # Errores
/// Falla si no se puede crear el directorio, el archivo o serializar los datos.
pub fn create_json(json_data: Value) -> ApiResponse {
    let path = get_json_path();

    if let Some(parent) = path.parent() {
        if !parent.exists() {
            if let Err(e) = fs::create_dir_all(parent) {
                return ApiResponse::new_error(
                    "Error al crear directorio".to_string(),
                    vec![e.to_string()],
                );
            }
        }
    }

    let json_string = match to_string_pretty(&json_data) {
        Ok(s) => s,
        Err(e) => {
            error!("Error al serializar JSON inicial: {}", e);
            return ApiResponse::new_error(
                "Error al serializar JSON".to_string(),
                vec![e.to_string()],
            );
        }
    };

    match File::create(&path) {
        Ok(mut file) => {
            if let Err(e) = file.write_all(json_string.as_bytes()) {
                error!("Error al escribir archivo JSON: {}", e);
                return ApiResponse::new_error(
                    "Error al escribir archivo".to_string(),
                    vec![e.to_string()],
                );
            }
            ApiResponse::new_success(json_data, "Archivo JSON creado exitosamente".to_string())
        }
        Err(e) => {
            error!("Error al crear archivo JSON: {}", e);
            ApiResponse::new_error("Error al crear archivo".to_string(), vec![e.to_string()])
        }
    }
}

/// Agrega un nuevo item al archivo JSON existente.
///
/// # Argumentos
/// * `key` - Clave del item a agregar.
/// * `value` - Valor del item a agregar.
///
/// # Retorno
/// Retorna una `ApiResponse` con el JSON actualizado o un error.
///
/// # Errores
/// Falla si el archivo no existe, no se puede leer/escribir o el JSON no es un objeto.
pub fn add_item(key: String, value: Value) -> ApiResponse {
    let path = get_json_path();
    if !path.exists() {
        return ApiResponse::new_error(
            "Archivo no encontrado".to_string(),
            vec!["Primero crea el archivo con create_json".to_string()],
        );
    }

    let mut json = match read_json_file(&path) {
        Ok(data) => data,
        Err(response) => return response,
    };

    if let Value::Object(ref mut map) = json {
        map.insert(key, value);
        write_json_file(&path, &json)
    } else {
        ApiResponse::new_error("El JSON raíz debe ser un objeto".to_string(), vec![])
    }
}

/// Elimina un item del archivo JSON existente.
///
/// # Argumentos
/// * `key` - Clave del item a eliminar.
///
/// # Retorno
/// Retorna una `ApiResponse` con el JSON actualizado o un error.
///
/// # Errores
/// Falla si el archivo no existe, no se puede leer/escribir o el JSON no es un objeto.
pub fn remove_item(key: String) -> ApiResponse {
    let path = get_json_path();
    if !path.exists() {
        return ApiResponse::new_error("Archivo no encontrado".to_string(), vec![]);
    }

    let mut json = match read_json_file(&path) {
        Ok(data) => data,
        Err(response) => return response,
    };

    if let Value::Object(ref mut map) = json {
        map.remove(&key);
        write_json_file(&path, &json)
    } else {
        ApiResponse::new_error("El JSON raíz debe ser un objeto".to_string(), vec![])
    }
}

/// Carga todos los datos del archivo JSON.
///
/// # Argumentos
///
/// # Retorno
/// Retorna una `ApiResponse` con los datos cargados o un error.
///
/// # Errores
/// Falla si el archivo no existe o no se puede leer.
pub fn load_data() -> ApiResponse {
    let path = get_json_path();
    if !path.exists() {
        return ApiResponse::new_error("Archivo no encontrado".to_string(), vec![]);
    }

    read_json_file(&path)
        .map(|data| ApiResponse::new_success(data, "Datos cargados exitosamente".to_string()))
        .unwrap_or_else(|response| response)
}

/// Lee el contenido del archivo JSON y lo deserializa.
///
/// # Argumentos
/// * `path` - Ruta del archivo JSON.
///
/// # Retorno
/// Retorna un `Result` con el `Value` deserializado o una `ApiResponse` con el error.
fn read_json_file(path: &PathBuf) -> Result<Value, ApiResponse> {
    let mut file = File::open(path).map_err(|e| {
        error!("Error al abrir archivo: {}", e);
        ApiResponse::new_error("Error al abrir archivo".to_string(), vec![e.to_string()])
    })?;

    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|e| {
        error!("Error al leer archivo: {}", e);
        ApiResponse::new_error("Error al leer archivo".to_string(), vec![e.to_string()])
    })?;

    from_str(&contents).map_err(|e| {
        error!("Error al parsear JSON: {}", e);
        ApiResponse::new_error("Error al parsear JSON".to_string(), vec![e.to_string()])
    })
}

/// Escribe datos JSON en el archivo especificado.
///
/// # Argumentos
/// * `path` - Ruta del archivo JSON.
/// * `json` - Datos JSON a escribir.
///
/// # Retorno
/// Retorna una `ApiResponse` con el resultado de la operación.
fn write_json_file(path: &PathBuf, json: &Value) -> ApiResponse {
    let json_string = match to_string_pretty(json) {
        Ok(s) => s,
        Err(e) => {
            error!("Error al serializar JSON: {}", e);
            return ApiResponse::new_error(
                "Error al serializar JSON".to_string(),
                vec![e.to_string()],
            );
        }
    };

    let mut file = match File::create(path) {
        Ok(file) => file,
        Err(e) => {
            error!("Error al abrir archivo para escritura: {}", e);
            return ApiResponse::new_error(
                "Error al abrir archivo para escritura".to_string(),
                vec![e.to_string()],
            );
        }
    };

    if let Err(e) = file.write_all(json_string.as_bytes()) {
        error!("Error al escribir archivo: {}", e);
        return ApiResponse::new_error(
            "Error al escribir archivo".to_string(),
            vec![e.to_string()],
        );
    }

    ApiResponse::new_success(
        json.clone(),
        "Operación completada exitosamente".to_string(),
    )
}
