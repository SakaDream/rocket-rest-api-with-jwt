use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::prelude::*;
use diesel::PgConnection;
use schema::users;
use schema::users::dsl::*;
use uuid::Uuid;
use models::login_history::LoginHistory;
use jwt::UserToken;

#[derive(Identifiable, Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub login_session: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct UserDTO {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginDTO {
    pub username_or_email: String,
    pub password: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct LoginInfoDTO {
    pub username: String,
    pub login_session: String,
}

impl User {
    pub fn signup(user: UserDTO, conn: &PgConnection) -> bool {
        let hashed_pwd = hash(&user.password, DEFAULT_COST).unwrap();
        let user = UserDTO {
            password: hashed_pwd,
            ..user
        };
        diesel::insert_into(users)
            .values(&user)
            .execute(conn)
            .is_ok()
    }

    pub fn login(login: LoginDTO, conn: &PgConnection) -> Option<LoginInfoDTO> {
        let user_to_verify = users
            .filter(username.eq(&login.username_or_email))
            .or_filter(email.eq(&login.username_or_email))
            .get_result::<User>(conn)
            .unwrap();
        if !user_to_verify.password.is_empty()
            && verify(&login.password, &user_to_verify.password).unwrap()
        {
            if let Some(login_history) = LoginHistory::create(&user_to_verify.username, conn) {
                if !LoginHistory::save_login_history(login_history, conn) {
                    return None;
                }
                let login_session_str = User::generate_login_session();
                User::update_login_session_to_db(&user_to_verify.username, &login_session_str, conn);
                Some(LoginInfoDTO {
                    username: user_to_verify.username,
                    login_session: login_session_str,
                })
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn is_valid_login_session(user_token: &UserToken, conn: &PgConnection) -> bool {
        users
            .filter(username.eq(&user_token.user))
            .filter(login_session.eq(&user_token.login_session))
            .get_result::<User>(conn)
            .is_ok()
    }

    pub fn find_user_by_username(un: &str, conn: &PgConnection) -> Option<User> {
        let result_user = users.filter(username.eq(un)).get_result::<User>(conn);
        if let Ok(user) = result_user {
            Some(user)
        } else {
            None
        }
    }

    pub fn generate_login_session() -> String {
        Uuid::new_v4().to_simple().to_string()
    }

    pub fn update_login_session_to_db(un: &str, login_session_str: &str, conn: &PgConnection) -> bool {
        if let Some(user) = User::find_user_by_username(un, conn) {
            diesel::update(users.find(user.id))
            .set(login_session.eq(login_session_str.to_string()))
            .execute(conn)
            .is_ok()
        } else {
            false
        }
    }
}
