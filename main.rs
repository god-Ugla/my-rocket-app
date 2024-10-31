#[macro_use]
extern crate rocket;

mod routes;

use rocket_dyn_templates::Template;
use rocket::fs::{FileServer, relative};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes::routes())
        .mount("/static", FileServer::from(relative!("static")))
        .attach(Template::fairing())
}
