use crate::models::response::ResponseBody;
use actix_web::{
    HttpResponse,
    error::ResponseError
};

// pub struct ServiceError {
//     pub http_status: StatusCode,
//     pub body: ResponseBody<String>,
// }

// impl ServiceError {
//     pub fn new(http_status: StatusCode, message: String) -> ServiceError {
//         ServiceError {
//             http_status,
//             body: ResponseBody {
//                 message,
//                 data: String::new(),
//             }
//         }
//     }

//     pub fn response(&self) -> HttpResponse {
//         HttpResponse::build(self.http_status).json(&self.body)
//     }
// }
use derive_more::Display;

#[derive(Debug, Display)]
pub enum ServiceError {
    #[display(fmt = "Internal Server Error: {}", _0)]
    InternalServerError(String),

    #[display(fmt - "Internal Server Error")]
    InternalServerErrorDefault,

    #[display(fmt = "BadRequest: {}", _0)]
    BadRequest(String),

    // #[display(fmt = "JWKSFetchError")]
    // JWKSFetchError,

    #[display(fmt = "JWTExpireError")]
    JWTExpireError,

    #[display(fmt = "UnauthorizedError: {}", _0)]
    UnauthorizedError(String),
}

// impl ResponseError trait allows to convert our errors into http responses with appropriate data
impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalServerError(ref message) => {
                HttpResponse::InternalServerError().json(ResponseBody{message: message.to_string(), data: ""})
            }
            ServiceError::InternalServerErrorDefault => {
                HttpResponse::InternalServerError().json(ResponseBody{message: "Internal Server Error, Please try later".to_string(), data: ""})
            }
            ServiceError::BadRequest(ref message) =>{
                HttpResponse::BadRequest().json(ResponseBody{message: message.to_string(), data: ""})
            }
            // ServiceError::JWKSFetchError => {
            //     HttpResponse::InternalServerError().json(ResponseBody{message: "Error fetching JWKS".to_string(), data: ""})
            // }
            ServiceError::JWTExpireError => {
                HttpResponse::Unauthorized().json(ResponseBody{message: "ExpiredJWT".to_string(), data: ""})
            }
            ServiceError::UnauthorizedError(ref message) =>{
                HttpResponse::Unauthorized().json(ResponseBody{message: message.to_string(), data: ""})
            }
        }
    }
}