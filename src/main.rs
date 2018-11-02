#![feature(proc_macro_hygiene, decl_macro)]

use rocket::request::Form;
use rocket::{get, routes, FromForm, State};
use std::sync::Mutex;

#[derive(FromForm, Debug)]
struct User {
    name: String,
    account: usize,
}

struct MyConfig {
    user_val: Mutex<u64>,
}

#[get("/item?<id>&<user..>")]
fn item(id: usize, user: Form<User>) -> String {
    println!("{}", id);
    user.into_inner().name
}

#[get("/<name>/<age>")]
fn hello(name: String, age: u8, state: State<MyConfig>) -> String {
    let mut data = state.user_val.lock().unwrap();
    *data += 1;
    format!("{}: Hello, {} year old named {}!", data, age, name)
}

#[get("/hello?<name>&<age>")]
fn hello_query(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[get("/hello")]
fn hello_world() -> &'static str {
    "Hello World!"
}

fn main() {
    let config = MyConfig {
        user_val: Mutex::new(0),
    };
    rocket::ignite()
        .mount("/", routes![hello, hello_world, hello_query, item])
        .manage(config)
        .launch();
}
