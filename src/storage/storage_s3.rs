use std::env;

pub fn s3_bucket() -> String {
    return env::var("S3_BUCKET").expect("Please set S3_BUCKET in .env file.").to_string();
}

pub fn s3_endpoint() -> String {
    return env::var("S3_ENDPOINT").expect("Please set S3_ENDPOINT in .env file.").to_string();
}

pub fn s3_user() -> String {
    return env::var("S3_USER").expect("Please set S3_USER in .env file.").to_string();
}

pub fn s3_pass() -> String {
    return env::var("S3_PASS").expect("Please set S3_PASS in .env file.").to_string();
}