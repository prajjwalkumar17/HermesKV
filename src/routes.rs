use actix_web::{web, HttpRequest, HttpResponse, Scope};
use prometheus::Counter;

use crate::{
    app::AppState,
    core::hermes::check_health,
    services::api::{http_response_err, http_response_json},
};

pub struct Hermes;

impl Hermes {
    pub fn server(state: AppState, health_check_counter: Counter) -> Scope {
        health_check_counter.inc();
        web::scope("/hermes")
            .app_data(web::Data::new(state))
            .service(web::resource("").route(web::get().to(server_health)))
    }
}

// #[utoipa::path(
//     get,
//     path = "/blocklist",
//     params (
//         ("data_kind" = BlocklistDataKind, Query, description = "Kind of the fingerprint list requested"),
//     ),
//     responses(
//         (status = 200, description = "Blocked Fingerprints", body = BlocklistResponse),
//         (status = 400, description = "Invalid Data")
//     ),
//     tag = "Blocklist",
//     operation_id = "List Blocked fingerprints of a particular kind",
//     security(("api_key" = []))
// )]
pub async fn server_health(_state: web::Data<AppState>, _req: HttpRequest) -> HttpResponse {
    // Box::pin(
    //     api::http_response_json(
    //     check_health(state).await))
    let json_response = check_health().await;
    match serde_json::to_string(&json_response) {
        Ok(res) => http_response_json(res),
        Err(_) => http_response_err(
            r#"{
                        "error": {
                            "message": "Error serializing response from connector"
                        }
                    }"#,
        ),
    }
}
