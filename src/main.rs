
// required for rocket macros to work
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;

mod db;
mod utils;
mod models;
mod schema;
// mod routes;
// mod models2;

type StdErr = Box<dyn std::error::Error>;

fn main() -> Result<(), StdErr> {
    // loads env variables from .env
    dotenv::dotenv()?;
    utils::logger::init()?;

    let db = db::Db::connect()?;

    rocket::ignite()
       .manage(db)
       .mount("/", rocket::routes![hello_world])
      //  .mount("/api", routes::api())
       .launch();

    // example
    assert_eq!("INFO", std::env::var("LOG_LEVEL").unwrap());

    Ok(())
}

#[rocket::get("/")]
fn hello_world() -> &'static str {
   "Hello, world!"
}