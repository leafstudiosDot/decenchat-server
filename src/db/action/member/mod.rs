use postgres::{Client, NoTls, Error};
use crate::db;

pub fn joinserver() {
    match crate::db::db_chose().as_str() {
        "POSTGRES" => {
            let db_client = Client::connect(
                format!("postgresql://{}:{}@{}:{}/postgres", db::db_postgres::pg_user(), db::db_postgres::pg_pass(), db::db_postgres::pg_host(), db::db_postgres::pg_port()).as_str(),
                NoTls,
            );
            //db_client.execute("")?;
        },
        _ => {
            println!("No database chosen");
        }
    }
}

pub fn leftserver() {
    match crate::db::db_chose().as_str() {
        "POSTGRES" => {
            
        },
        _ => {
            println!("No database chosen");
        }
    }
}