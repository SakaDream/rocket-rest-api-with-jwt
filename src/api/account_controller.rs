use config::DbConn;
use models::response::Response;
use models::user::{LoginDTO, UserDTO};
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;
use services::account_service;

#[post("/signup", format = "json", data = "<user>")]
pub fn signup(user: Json<UserDTO>, conn: DbConn) -> status::Custom<Json<Response>> {
    let response = account_service::signup(user.0, conn);
    status::Custom(
        Status::from_code(response.status_code).unwrap(),
        Json(response.response),
    )
}

#[post("/login", format = "json", data = "<login>")]
pub fn login(login: Json<LoginDTO>, conn: DbConn) -> status::Custom<Json<Response>> {
    let response = account_service::login(login.0, conn);
    status::Custom(
        Status::from_code(response.status_code).unwrap(),
        Json(response.response),
    )
}
