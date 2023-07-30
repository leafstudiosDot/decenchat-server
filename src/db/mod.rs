use std::env;

pub fn db_chose() -> String {
    return env::var("DB").expect("Please set DB in .env file. Possible choices are [POSTGRES]").to_string();
}

pub mod db_postgres;
pub mod action;