use crate::state::AppState;
use actix_web::{get, web};
use std::sync::atomic;

pub(crate) fn service(config: &mut web::ServiceConfig) {
    config.service(get_metrics);
}

#[get("")]
async fn get_metrics(app_state: web::Data<AppState>) -> String {
    let mut metrics = Vec::new();

    // Requests
    let bot_requests = app_state.stats.bot_requests.load(atomic::Ordering::Relaxed);
    metrics.push(format!(
        "ai_hole_requests_processed{{role=\"bot\"}} {}",
        bot_requests
    ));

    let human_requests = app_state
        .stats
        .human_requests
        .load(atomic::Ordering::Relaxed);
    metrics.push(format!(
        "ai_hole_requests_processed{{role=\"human\"}} {}",
        human_requests
    ));

    // Elements
    let elements_removed = app_state
        .stats
        .elements_removed
        .load(atomic::Ordering::Relaxed);
    metrics.push(format!(
        "ai_hole_elements_processed{{action=\"remove\"}} {}",
        elements_removed
    ));

    let elements_randomized = app_state
        .stats
        .elements_randomized
        .load(atomic::Ordering::Relaxed);
    metrics.push(format!(
        "ai_hole_elements_processed{{action=\"randomized\"}} {}",
        elements_randomized
    ));

    metrics.join("\n")
}
