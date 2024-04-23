use std::path::Path;

use chrono::{DateTime, Utc};
use email_address::EmailAddress;
use rocket::{Build, Rocket};

#[derive(Debug, Clone)]
pub struct User {
    uid: u128,
    user_name: Box<str>,
    mail_id: EmailAddress,
}

#[derive(Debug, Clone)]
pub enum Content {
    Text(Box<str>),
    Image(Box<Path>),
    Video(Box<Path>),
}

#[derive(Debug, Clone)]
pub struct Message {
    sender_uid: u128,
    message_id: u128,
    timestamp: DateTime<Utc>,
    content: Content,
}

#[derive(Debug, Clone, Default)]
pub struct Chat {
    chat_id: u128,
    member_uids: Vec<u128>,
    messages: Vec<Message>,
}

#[rocket::get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[rocket::launch]
fn launch() -> Rocket<Build> {
    rocket::build().mount("/hello", rocket::routes![index])
}
