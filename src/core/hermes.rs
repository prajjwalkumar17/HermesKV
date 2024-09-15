use crate::{
    api_models::response_models::CheckHealthResponse,
    errors::{Response, RouterResponse},
};

pub async fn check_health(// _state: AppState,
    // merchant_account: domain::MerchantAccount,
    // body: api_blocklist::AddToBlocklistRequest,
) -> RouterResponse<CheckHealthResponse> {
    let result = Ok(CheckHealthResponse {
        status: 200,
        response: "Health is perfect!!!".to_string(),
    });
    Ok(result.map(Response::Json))
}
