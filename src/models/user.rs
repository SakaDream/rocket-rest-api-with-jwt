use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::prelude::*;
use diesel::PgConnection;
use schema::users;
use schema::users::dsl::*;

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    id: i32,
    username: String,
    email: String,
    password: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct UserDTO {
    username: String,
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginDTO {
    username_or_email: String,
    password: String,
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

    pub fn login(login: LoginDTO, conn: &PgConnection) -> String {
        let user_to_verify = users
            .filter(username.eq(&login.username_or_email))
            .or_filter(email.eq(&login.username_or_email))
            .get_result::<User>(conn)
            .unwrap();
        if !user_to_verify.password.is_empty()
            && verify(&login.password, &user_to_verify.password).unwrap()
        {
            user_to_verify.username
        } else {
            String::new()
        }
    }

    pub fn is_user_exists(un: String, conn: &PgConnection) -> bool {
        let user = users.filter(username.eq(&un)).get_result::<User>(conn);
        if user.is_err() {
            false
        } else {
            true
        }
    }
}
