use actix_web::{
    body::{BoxBody, MessageBody},
    dev::{Server, ServiceFactory, ServiceRequest, ServiceResponse},
    http::StatusCode,
    middleware,
};

use crate::{
    config::{self, GlobalConfig},
    errors::ApplicationResult,
    middlewares, routes,
};

pub fn check_health() {
    println!("Health is good!");
}

#[derive(Clone)]
pub struct AppState {
    pub conf: GlobalConfig,
}

impl AppState {
    pub fn new(conf: GlobalConfig) -> Self {
        Self { conf }
    }
}

pub async fn server_builder(conf: GlobalConfig) -> ApplicationResult<Server> {
    let server = conf.server.clone();
    let state = AppState::new(conf);
    let request_body_limit = server.request_body_limit;

    let server_builder =
        actix_web::HttpServer::new(move || mk_app(state.clone(), request_body_limit.into()))
            .bind((server.host.as_str(), server.port))?;
    Ok(server_builder.run())
}

pub fn mk_app(
    state: AppState,
    request_body_limit: usize,
) -> actix_web::App<
    impl ServiceFactory<
        ServiceRequest,
        Config = (),
        Response = actix_web::dev::ServiceResponse<impl MessageBody>,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    let mut server_app = get_application_builder(request_body_limit);
    server_app = server_app.service(routes::Hermes::server(state));
    server_app
}

pub fn get_application_builder(
    request_body_limit: usize,
) -> actix_web::App<
    impl ServiceFactory<
        ServiceRequest,
        Config = (),
        Response = actix_web::dev::ServiceResponse<impl MessageBody>,
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
