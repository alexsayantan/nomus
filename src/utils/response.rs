use actix_web::{body::BoxBody, http::StatusCode, web, HttpResponse, Responder};

pub struct ApiResponse {
    pub message: String,
    response_code: StatusCode,
}

impl ApiResponse {
    pub fn new(status_code: u16, message: String) -> Self {
        ApiResponse {
            message,
            response_code: StatusCode::from_u16(status_code).unwrap(),
        }
    }
}

impl Responder for ApiResponse {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        let body = BoxBody::new(web::BytesMut::from(self.message.as_bytes()));
        HttpResponse::new(self.response_code).set_body(body)
    }
}