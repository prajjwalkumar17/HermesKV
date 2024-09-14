use actix_web::{
    dev::{ServiceFactory, ServiceRequest, ServiceResponse},
    body::{MessageBody, BoxBody}, middleware, http::StatusCode};

use crate::{config, middlewares};

pub fn check_health() {
    println!("Health is good!");
}

pub struct AppState {
    pub conf: config::GlobalConfig,
}

pub fn server_builder() {


}

// pub fn mk_app (
//     state: AppState,
//     req_size: u16,
// ) -> actix_web::App<
//     impl ServiceFactory<
//         ServiceRequest,
//         Config = (),
//         Response = actix_web::dev::Response<impl MessageBody>,
//         Error = actix_web::Error,
//         InitError = (),
//     >,
//   >
// {
//
// }

pub fn get_application_builder(
    request_body_limit: usize,
) -> actix_web::App<
    impl ServiceFactory<
        ServiceRequest,
        Config = (),
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    let json_config = actix_web::web::JsonConfig::default()
        .limit(request_body_limit)
        .content_type_required(true);
    
    let server = actix_web::App::new()
        .app_data(json_config)
        .wrap(middlewares::default_response_headers())
        .wrap(middleware::Logger::default());
    
    server
}
