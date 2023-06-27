use crate::config::Config;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use actix_web::dev::Payload;
use actix_web::error::ErrorServiceUnavailable;
use actix_web::{Error, FromRequest, HttpRequest};
use futures::future::{err, ok, Ready};

use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool, PooledConnection},
};
use lazy_static::lazy_static;

type PgPool = Pool<ConnectionManager<PgConnection>>;

pub struct DbConnection {
    pub db_connection: PgPool,
}

// ---- sets up a db connection pool -----
lazy_static! {
    pub static ref DBCONNECTION: DbConnection = {
        let connection_string = Config::new()
            .map
            .get("DB_URL")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
        println!("{}", connection_string);
        DbConnection {
            db_connection: PgPool::builder()
                .max_size(8)
                .build(ConnectionManager::new(connection_string))
                .expect("Failed to create db connection_pool"),
        }
    };
}

pub fn establish_connection() -> PooledConnection<ConnectionManager<PgConnection>> {
    return DBCONNECTION.db_connection.get().unwrap();

    // no connection pool
    // dotenv().ok();car

    // let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // PgConnection::establish(&database_url)
    //     .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub struct DB {
    pub connection: PooledConnection<ConnectionManager<PgConnection>>,
}

impl FromRequest for DB {
    type Error = Error;
    type Future = Ready<Result<DB, Error>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        match DBCONNECTION.db_connection.get() {
            Ok(connection) => {
                return ok(DB { connection });
            }
            Err(_) => {
                return err(ErrorServiceUnavailable(
                    "Could not make connection to database",
                ))
            }
        }
    }
}
