use crate::config::AppConfig;
use actix_web::{web, Responder};
use std::sync::Mutex;

// Handler for getting units configuration
async fn get_units_config(data: web::Data<Mutex<AppConfig>>) -> impl Responder {
    let config = data.lock().unwrap();
    web::Json(config.units.clone())
}

// Handler for getting counters configuration
async fn get_counters_config(data: web::Data<Mutex<AppConfig>>) -> impl Responder {
    let config = data.lock().unwrap();
    web::Json(config.counters.clone())
}

// Handler for getting cost types configuration
async fn get_cost_types_config(data: web::Data<Mutex<AppConfig>>) -> impl Responder {
    let config = data.lock().unwrap();
    web::Json(config.cost_types.clone())
}

// Handler for getting heizung params configuration
async fn get_heizung_params_config(data: web::Data<Mutex<AppConfig>>) -> impl Responder {
    let config = data.lock().unwrap();
    web::Json(config.heizung_config.clone())
}

// Initialize routes for configuration endpoints
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/config")
            .route("/units", web::get().to(get_units_config))
            .route("/counters", web::get().to(get_counters_config))
            .route("/cost_types", web::get().to(get_cost_types_config))
            .route("/heizung_params", web::get().to(get_heizung_params_config)),
    );
}
