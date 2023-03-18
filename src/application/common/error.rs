use serde::Serialize;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Serialize)]
pub struct CommonError {
    pub message: String,
    pub code: u32,
}

impl Display for CommonError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Error: {}, Code: {}", self.message, self.code)
    }
}

#[derive(Debug)]
pub struct ApiError(CommonError);

impl From<CommonError> for ApiError {
    fn from(error: CommonError) -> ApiError {
        ApiError(error)
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Error: {}, Code: {}", self.0.message, self.0.code)
    }
}

impl actix_web::ResponseError for ApiError {
    fn error_response(&self) -> actix_web::HttpResponse {
        actix_web::HttpResponse::BadRequest().json(&self.0)
    }
}

#[derive(Debug)]
pub struct RepositoryError {
    pub message: String,
}

impl Into<CommonError> for RepositoryError {
    fn into(self) -> CommonError {
        CommonError {
            message: self.message,
            code: 1,
        }
    }
}
