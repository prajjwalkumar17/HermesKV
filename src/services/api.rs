use actix_web::{body, HttpMessage, HttpResponse};

pub fn http_response_json<T: body::MessageBody + 'static>(resonse: T) -> HttpResponse {
    HttpResponse::Ok()
        .content_type(mime::APPLICATION_JSON)
        .body(resonse)
}

pub fn http_response_err<T: body::MessageBody + 'static>(response: T) -> HttpResponse {
    HttpResponse::BadRequest()
        .content_type(mime::APPLICATION_JSON)
        .body(response)
}
