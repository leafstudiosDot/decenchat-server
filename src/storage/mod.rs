use std::env;

pub fn storage_chose() -> String {
    return env::var("UGC_STORAGE").expect("Please set UGC_STORAGE in .env file. Possible choices are [S3]").to_string();
}

pub mod storage_s3;