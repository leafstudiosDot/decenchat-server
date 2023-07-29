use std::env;

pub fn pg_host() -> String {
    return env::var("POSTGRES_HOST").expect("Please set POSTGRES_HOST in .env file.").to_string();
}

pub fn pg_port() -> String {
    return env::var("POSTGRES_PORT").expect("Please set POSTGRES_PORT in .env file.").to_string();
}

pub fn pg_user() -> String {
    return env::var("POSTGRES_USER").expect("Please set POSTGRES_USER in .env file.").to_string();
}

pub fn pg_pass() -> String {
    return env::var("POSTGRES_PASS").expect("Please set POSTGRES_PASS in .env file.").to_string();
}