use config::DbConn;
use constants::message_constants;
use jsonwebtoken::errors::Result;
use jsonwebtoken::TokenData;
use jsonwebtoken::{Header, Validation};
use models::response::Response;
use models::user::User;
use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::{self, FromRequest, Request};
use rocket::response::status;
use rocket_contrib::json::Json;

static KEY: &'static [u8; 16] = include_bytes!("secret.key");
static ONE_WEEK: i64 = 60 * 60 * 24 * 7;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserToken {
    // issued at
    iat: i64,
    // expiration
    exp: i64,
    user: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for UserToken {
    type Error = status::Custom<Json<Response>>;
    fn from_request(
        request: &'a Request<'r>,
    ) -> request::Outcome<Self, status::Custom<Json<Response>>> {
        let conn = request.guard::<DbConn>().unwrap();
        if let Some(authen_header) = request.headers().get_one("Authorization") {
            let authen_str = authen_header.to_string();
            if authen_str.starts_with("bearer") {
                let token = authen_str[6..authen_str.len()].trim();
                if let Ok(token_data) = decode_token(token.to_string()) {
                    if verify_token(&token_data, &conn) {
                        return Outcome::Success(token_data.claims);
                    }
                } 
            }
        }

        return Outcome::Failure((
            Status::BadRequest,
            status::Custom(
                Status::BadRequest,
                Json(Response {
                    message: String::from(message_constants::MESSAGE_INVALID_TOKEN),
                    data: serde_json::to_value("").unwrap(),
                }),
            ),
        ));
    }
}

pub fn generate_token(user: String) -> String {
    let now = time::get_time().sec;
    let payload = UserToken {
        iat: now,
        exp: now + ONE_WEEK,
        user: user,
    };

    jsonwebtoken::encode(&Header::default(), &payload, KEY).unwrap()
}

fn decode_token(token: String) -> Result<TokenData<UserToken>> {
    jsonwebtoken::decode::<UserToken>(&token, KEY, &Validation::default())
}

fn verify_token(token_data: &TokenData<UserToken>, conn: &DbConn) -> bool {
    User::is_user_exists(token_data.claims.user.to_string(), conn)
}
