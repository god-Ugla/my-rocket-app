use rocket::serde::json::Json;
use rocket_dyn_templates::Template;
use std::collections::HashMap;

#[get("/")]
fn index() -> Template {
    let context: HashMap<&str, &str> = HashMap::new();
    Template::render("index", &context)
}

#[get("/about")]
fn about() -> &'static str {
    "This is the About page"
}

#[get("/api/data")]
fn api_data() -> Json<HashMap<&'static str, &'static str>> {
    let mut data = HashMap::new();
    data.insert("name", "Rocket Website");
    Json(data)
}

pub fn routes() -> Vec<rocket::Route> {
    routes![index, about, api_data]
}