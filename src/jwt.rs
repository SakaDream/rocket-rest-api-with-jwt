use chrono::Utc;
use config::DbConn;
use constants::message_constants;
use jsonwebtoken::errors::Result;
use jsonwebtoken::TokenData;
use jsonwebtoken::{Header, Validation};
use jsonwebtoken::{EncodingKey, DecodingKey};
use models::response::Response;
use models::user::{ User, LoginInfoDTO };
use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::{self, FromRequest, Request};
use rocket::response::status;
use rocket_contrib::json::Json;

static ONE_WEEK: i64 = 60 * 60 * 24 * 7; // in seconds

#[derive(Debug, Serialize, Deserialize)]
pub struct UserToken {
    // issued at
    pub iat: i64,
    // expiration
    pub exp: i64,
    // data
    pub user: String,
    pub login_session: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for UserToken {
    type Error = status::Custom<Json<Response>>;
    fn from_request(
        request: &'a Request<'r>,
    ) -> request::Outcome<Self, status::Custom<Json<Response>>> {
        let conn = request.guard::<DbConn>().unwrap();
        if let Some(authen_header) = request.headers().get_one("Authorization") {
            let authen_str = authen_header.to_string();
            if authen_str.starts_with("Bearer") {
                let token = authen_str[6..authen_str.len()].trim();
                if let Ok(token_data) = decode_token(token.to_string()) {
                    if verify_token(&token_data, &conn) {
                        return Outcome::Success(token_data.claims);
                    }
                } 
            }
        }

        Outcome::Failure((
            Status::BadRequest,
            status::Custom(
                Status::Unauthorized,
                Json(Response {
                    message: String::from(message_constants::MESSAGE_INVALID_TOKEN),
                    data: serde_json::to_value("").unwrap(),
                }),
            ),
        ))
    }
}

pub fn generate_token(login: LoginInfoDTO) -> String {
    let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nanosecond -> second
    let payload = UserToken {
        iat: now,
        exp: now + ONE_WEEK,
        user: login.username,
        login_session: login.login_session,
    };

    jsonwebtoken::encode(&Header::default(), &payload, &EncodingKey::from_secret(include_bytes!("secret.key"))).unwrap()
}

fn decode_token(token: String) -> Result<TokenData<UserToken>> {
    jsonwebtoken::decode::<UserToken>(&token, &DecodingKey::from_secret(include_bytes!("secret.key")), &Validation::default())
}

fn verify_token(token_data: &TokenData<UserToken>, conn: &DbConn) -> bool {
    User::is_valid_login_session(&token_data.claims, conn)
}
