use postgres::{Client, NoTls, Error};

pub fn joinserver() {
    match crate::db::db_chose().as_str() {
        "POSTGRES" => {
            
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