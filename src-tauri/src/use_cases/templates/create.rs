use crate::adapters::dto::template_dto::TemplateDto;
use crate::core::entities::Template;
use crate::core::repository::template_repository::TemplateRepository;
use crate::shared::response::ApiResponse;

pub fn new(dto: TemplateDto,  repository: &mut dyn TemplateRepository) -> ApiResponse {
    // Convertir TemplateDto a Template
    // Convert TemplateDto to Template
    let template = match Template::try_from(dto) {
        Ok(t) => t,
        Err(e) => return ApiResponse::new_error("Datos inválidos".to_string(), vec![e]),
    };

    // Call the repository's create method
    match repository.create(&template) {
        Ok(()) => {
            ApiResponse::new_success(serde_json::json!({}), "Creado Correctamente".to_string())
        } // Adjust based on ApiResponse definition
        Err(e) => ApiResponse::new_error(e, vec!["Error al crear el template".to_string()]), // Adjust based on ApiResponse definition
    }
}
