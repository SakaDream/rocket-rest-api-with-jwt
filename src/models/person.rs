use diesel::prelude::*;
use diesel::PgConnection;
use schema::people;
use schema::people::dsl::*;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Person {
    pub id: i32,
    pub name: String,
    pub gender: bool,
    pub age: i32,
    pub address: String,
    pub phone: String,
    pub email: String,
}

#[derive(Insertable, AsChangeset, Serialize, Deserialize)]
#[table_name = "people"]
pub struct PersonDTO {
    pub name: String,
    pub gender: bool,
    pub age: i32,
    pub address: String,
    pub phone: String,
    pub email: String,
}

impl Person {
    pub fn find_all(conn: &PgConnection) -> Vec<Person> {
        people.order(id.asc()).load::<Person>(conn).unwrap()
    }

    pub fn find_by_id(i: i32, conn: &PgConnection) -> Option<Person> {
        let result_p = people.find(i).get_result::<Person>(conn);
        if let Ok(p) = result_p {
            Some(p)
        } else {
            None
        }
    }

    pub fn query(query: String, conn: &PgConnection) -> Vec<Person> {
        let pattern = format!("%{}%", query);
        let mut id_and_age_query: i32 = 0;
        let mut id_and_age_query_flag = false;
        if query.as_str().parse::<i32>().is_ok() {
            id_and_age_query_flag = true;
            id_and_age_query = query.as_str().parse::<i32>().unwrap();
        }

        let gender_query;
        let gender_query_flag;
        match query.to_lowercase().as_str() {
            "male" => {
                gender_query = true;
                gender_query_flag = true;
            }
            "female" => {
                gender_query = false;
                gender_query_flag = true;
            }
            _ => {
                gender_query = false;
                gender_query_flag = false;
            }
        }

        if id_and_age_query_flag && gender_query_flag {
            people
                .order(id.asc())
                .filter(id.eq(&id_and_age_query))
                .or_filter(name.like(&pattern))
                .or_filter(gender.eq(&gender_query))
                .or_filter(age.eq(&id_and_age_query))
                .or_filter(address.like(&pattern))
                .load::<Person>(conn)
                .unwrap()
        } else if id_and_age_query_flag && !gender_query_flag {
            people
                .order(id.asc())
                .filter(id.eq(&id_and_age_query))
                .or_filter(name.like(&pattern))
                .or_filter(age.eq(&id_and_age_query))
                .or_filter(address.like(&pattern))
                .load::<Person>(conn)
                .unwrap()
        } else if !id_and_age_query_flag && gender_query_flag {
            people
                .order(id.asc())
                .filter(name.like(&pattern))
                .or_filter(gender.eq(&gender_query))
                .or_filter(address.like(&pattern))
                .load::<Person>(conn)
                .unwrap()
        } else {
            people
                .order(id.asc())
                .filter(name.like(&pattern))
                .or_filter(address.like(&pattern))
                .load::<Person>(conn)
                .unwrap()
        }
    }

    pub fn insert(new_person: PersonDTO, conn: &PgConnection) -> bool {
        diesel::insert_into(people)
            .values(&new_person)
            .execute(conn)
            .is_ok()
    }

    pub fn update(i: i32, updated_person: PersonDTO, conn: &PgConnection) -> bool {
        diesel::update(people.find(i))
            .set(&updated_person)
            .execute(conn)
            .is_ok()
    }

    pub fn delete(i: i32, conn: &PgConnection) -> bool {
        diesel::delete(people.find(i)).execute(conn).is_ok()
    }
}
