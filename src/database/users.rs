use crate::models::user::User;
use crate::schema::users;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error};
use scrypt::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Scrypt,
};
use serde::Deserialize;

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub hash: &'a str,
}

pub enum UserCreationError {
    DuplicatedEmail,
    DuplicatedUsername,
}

impl From<Error> for UserCreationError {
    fn from(err: Error) -> UserCreationError {
        if let Error::DatabaseError(DatabaseErrorKind::UniqueViolation, info) = &err {
            match info.constraint_name() {
                Some("users_username_key") => return UserCreationError::DuplicatedUsername,
                Some("users_email_key") => return UserCreationError::DuplicatedEmail,
                _ => {}
            }
        }
        panic!("Error creating user: {:?}", err)
    }
}

pub fn create(
    conn: &mut MysqlConnection,
    username: &str,
    email: &str,
    password: &str,
) -> Result<User, UserCreationError> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Scrypt
        .hash_password(password.as_bytes(), &salt)
        .expect("hash error")
        .to_string()
        .to_owned();

    let new_user = &NewUser {
        username,
        email,
        hash: &hash[..],
    };
    conn.transaction::<User, Error, _>(|conn| {
        diesel::insert_into(users::table)
            .values(new_user)
            .execute(conn)?;
        // .map_err(Into::into)
        users::table.order(users::id.desc()).first(conn)
        // match created_user {
        //     Ok(user) => return Ok(user),
        //     Err(err) => return Err(err.into()),
        // }
    }).map_err(|err|err.into())
}

pub fn login(conn: &mut MysqlConnection, email: &str, password: &str) -> Option<User> {
    let user = users::table
        .filter(users::email.eq(email))
        .get_result::<User>(conn)
        .map_err(|err| eprintln!("login_user: {}", err))
        .ok()?;

    let parsed_hash = PasswordHash::new(&user.hash).unwrap();
    let password_matches = Scrypt
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|err| eprintln!("login_user: scrypt_check: {}", err))
        .is_ok();

    if password_matches {
        Some(user)
    } else {
        eprintln!(
            "login attempt for '{}' failed: password doesn't match",
            email
        );
        None
    }
}

pub fn find(conn: &mut MysqlConnection, id: i32) -> Option<User> {
    users::table
        .find(id)
        .get_result(conn)
        .map_err(|err| println!("find_user: {}", err))
        .ok()
}

// TODO: remove clone when diesel will allow skipping fields
#[derive(Deserialize, AsChangeset, Default, Clone)]
#[diesel(table_name = users)]
pub struct UpdateUserData {
    username: Option<String>,
    email: Option<String>,
    bio: Option<String>,
    image: Option<String>,

    // hack to skip the field
    #[diesel(column_name = hash)]
    password: Option<String>,
}

pub fn update(conn: &mut MysqlConnection, id: i32, data: &UpdateUserData) -> Option<User> {
    let data = &UpdateUserData {
        password: None,
        ..data.clone()
    };
    let user = conn.transaction::<User, Error, _>(|conn| {
        diesel::update(users::table.find(id))
            .set(data)
            .execute(conn)?;
        users::table.find(id).get_result(conn)
    });
    Some(user.expect("error"))
}
