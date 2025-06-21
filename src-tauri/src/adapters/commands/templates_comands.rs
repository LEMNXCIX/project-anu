use tauri::{command, State};
use crate::adapters::dto::template_dto::TemplateDto;
use crate::shared::response::ApiResponse;
use crate::state::AppState;
use crate::use_cases::templates::create::new;


#[command]
pub fn create_template_command(
    state: State<'_, AppState>,
    dto: TemplateDto,
) -> ApiResponse {
    let mut repo = state.template_repository.lock().unwrap();
    new(dto, &mut **repo)
}