use config::DbConn;
use constants::message_constants;
use models::person::{Person, PersonDTO};
use models::response::{Response, ResponseWithStatus};
use rocket::http::Status;

pub fn find_all(conn: DbConn) -> ResponseWithStatus {
    ResponseWithStatus {
        status_code: Status::Ok.code,
        response: Response {
            message: String::from(message_constants::MESSAGE_OK),
            data: serde_json::to_value(Person::find_all(&conn)).unwrap(),
        },
    }
}

pub fn find_by_id(id: i32, conn: DbConn) -> ResponseWithStatus {
    let option_person = Person::find_by_id(id, &conn);
    if let Some(person) = option_person {
        ResponseWithStatus {
            status_code: Status::Ok.code,
            response: Response {
                message: String::from(message_constants::MESSAGE_OK),
                data: serde_json::to_value(person).unwrap(),
            },
        }
    } else {
        ResponseWithStatus {
            status_code: Status::NotFound.code,
            response: Response {
                message: format!("person with id {} not found", id),
                data: serde_json::to_value("").unwrap(),
            },
        }
    }
}

pub fn query(query: String, conn: DbConn) -> ResponseWithStatus {
    ResponseWithStatus {
        status_code: Status::Ok.code,
        response: Response {
            message: String::from(message_constants::MESSAGE_OK),
            data: serde_json::to_value(Person::query(query, &conn)).unwrap(),
        },
    }
}

pub fn insert(new_person: PersonDTO, conn: DbConn) -> ResponseWithStatus {
    if Person::insert(new_person, &conn) {
        ResponseWithStatus {
            status_code: Status::Created.code,
            response: Response {
                message: String::from(message_constants::MESSAGE_OK),
                data: serde_json::to_value("").unwrap(),
            },
        }
    } else {
        ResponseWithStatus {
            status_code: Status::InternalServerError.code,
            response: Response {
                message: String::from(message_constants::MESSAGE_CAN_NOT_INSERT_DATA),
                data: serde_json::to_value("").unwrap(),
            },
        }
    }
}

pub fn update(id: i32, updated_person: PersonDTO, conn: DbConn) -> ResponseWithStatus {
    let option_person = Person::find_by_id(id, &conn);
    if option_person.is_some() {
        if Person::update(id, updated_person, &conn) {
            ResponseWithStatus {
                status_code: Status::Ok.code,
                response: Response {
                    message: String::from(message_constants::MESSAGE_OK),
                    data: serde_json::to_value("").unwrap(),
                },
            }
        } else {
            ResponseWithStatus {
                status_code: Status::InternalServerError.code,
                response: Response {
                    message: String::from(message_constants::MESSAGE_CAN_NOT_UPDATE_DATA),
                    data: serde_json::to_value("").unwrap(),
                },
            }
        }
    } else {
        ResponseWithStatus {
            status_code: Status::InternalServerError.code,
            response: Response {
                message: format!("person with id {} not found", id),
                data: serde_json::to_value("").unwrap(),
            },
        }
    }
}

pub fn delete(id: i32, conn: DbConn) -> ResponseWithStatus {
    let option_person = Person::find_by_id(id, &conn);
    if option_person.is_some() {
        if Person::delete(id, &conn) {
            ResponseWithStatus {
                status_code: Status::Ok.code,
                response: Response {
                    message: String::from(message_constants::MESSAGE_OK),
                    data: serde_json::to_value("").unwrap(),
                },
            }
        } else {
            ResponseWithStatus {
                status_code: Status::BadRequest.code,
                response: Response {
                    message: format!("person with id {} not found", id),
                    data: serde_json::to_value("").unwrap(),
                },
            }
        }
    } else {
        ResponseWithStatus {
            status_code: Status::InternalServerError.code,
            response: Response {
                message: String::from(message_constants::MESSAGE_CAN_NOT_DELETE_DATA),
                data: serde_json::to_value("").unwrap(),
            },
        }
    }
}
