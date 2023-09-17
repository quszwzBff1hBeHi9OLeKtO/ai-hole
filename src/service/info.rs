use crate::state::AppState;
use actix_web::{get, web};

pub(crate) fn service(config: &mut web::ServiceConfig) {
    config.service(get_index);
}

#[get("/")]
async fn get_index(app_state: web::Data<AppState>) -> String {
    let config = serde_yaml::to_string(&app_state.config).expect("failed to serialize config");

    let version = env!("CARGO_PKG_VERSION");

    format!("AI-Hole v{version}\n\nConfig:\n-------\n{config}")
}
