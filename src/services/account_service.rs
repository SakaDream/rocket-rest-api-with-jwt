use config::DbConn;
use constants::message_constants;
use jwt;
use models::response::{Response, ResponseWithStatus};
use models::user::{LoginDTO, User, UserDTO};
use rocket::http::Status;

pub fn signup(user: UserDTO, conn: DbConn) -> ResponseWithStatus {
    if User::signup(user, &conn) {
        ResponseWithStatus {
            status_code: Status::Ok.code,
            response: Response {
                message: String::from(message_constants::MESSAGE_SIGNUP_SUCCESS),
                data: serde_json::to_value("").unwrap(),
            },
        }
    } else {
        ResponseWithStatus {
            status_code: Status::BadRequest.code,
            response: Response {
                message: String::from(message_constants::MESSAGE_SIGNUP_FAILED),
                data: serde_json::to_value("").unwrap(),
            },
        }
    }
}

pub fn login(login: LoginDTO, conn: DbConn) -> ResponseWithStatus {
    if let Some(result) = User::login(login, &conn) {
        ResponseWithStatus {
            status_code: Status::Ok.code,
            response: Response {
                message: String::from(message_constants::MESSAGE_LOGIN_SUCCESS),
                data: serde_json::to_value(json!({ "token": jwt::generate_token(result), "type": "Bearer" }))
                    .unwrap(),
            },
        }
    } else {
        ResponseWithStatus {
            status_code: Status::BadRequest.code,
            response: Response {
                message: String::from(message_constants::MESSAGE_LOGIN_FAILED),
                data: serde_json::to_value("").unwrap(),
            },
        }
    }
}
