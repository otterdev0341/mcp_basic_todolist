use std::fmt;

use rocket::{http::Status, Responder};


#[derive(Responder)]
pub struct SuccessResponse<T>(pub (Status, T));

#[derive(Responder, Debug)]
pub struct ErrorResponse (pub (Status, String));




pub type Response<T> = Result<SuccessResponse<T>, ErrorResponse>;

// impl From<DbErr> for ErrorResponse {
//     fn from(err: DbErr) -> Self {
//         ErrorResponse((Status::InternalServerError, format!("Database error: {:?}", err)))
//     }
// }

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ErrorResponse: status = {}, message = {}", (self.0).0, (self.0).1)
    }
}