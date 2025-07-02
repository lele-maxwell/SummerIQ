use actix_web::web;
use crate::handlers::documentation::get_project_documentation;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/api/documentation/project/{path:.*}")
            .route(web::get().to(get_project_documentation)),
    );
} 