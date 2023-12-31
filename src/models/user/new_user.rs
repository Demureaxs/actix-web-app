use crate::schema::users;
use bcrypt::{hash, DEFAULT_COST};
use diesel::Insertable;
use uuid::Uuid;

#[derive(Insertable, Clone)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub unique_id: String,
}
impl NewUser {
    pub fn new(username: String, email: String, password: String) -> NewUser {
        // bcrypt hashes the password for the new user
        let hashed_password: String = hash(password.as_str(), DEFAULT_COST).unwrap();
        // uuid generates a unique user id
        let uuid = Uuid::new_v4().to_string();
        
        //returns the NewUser Struct
        return NewUser {
            username,
            email,
            password: hashed_password,
            unique_id: uuid,
        };
    }
}
