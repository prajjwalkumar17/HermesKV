/// Middleware for attaching default response headers. Headers with the same key already set in a
/// response will not be overwritten.
pub fn default_response_headers() -> actix_web::middleware::DefaultHeaders {
    use actix_web::http::header;

    let default_headers_middleware = actix_web::middleware::DefaultHeaders::new();

    default_headers_middleware
        .add((header::STRICT_TRANSPORT_SECURITY, "max-age=31536000"))
        .add((header::VIA, "HermesKv"))
}
