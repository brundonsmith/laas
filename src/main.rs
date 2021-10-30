use life::{Coord, LifeState};

#[macro_use]
extern crate rocket;

mod life;

use rocket::response::content::Html;
use rocket::response::status::BadRequest;

// entry point
#[launch]
fn rocket() -> _ {
    let port: u16 = std::env::var_os("PORT").unwrap().to_str().unwrap().parse().unwrap();

    rocket::custom(rocket::Config::figment().merge(("port", port)))
        .mount("/", routes![life_raw, life_html])
}

// routes
#[get("/<current>")]
fn life_raw(current: LifeState) -> Result<LifeState, BadRequest<String>> {
    Ok(current.next_state())
}

#[get("/html/<current>")]
fn life_html(current: LifeState) -> Result<Html<String>, BadRequest<String>> {
    let next_state = current.next_state();

    let mut html = String::new();

    html += "<body>";

    html += &format!("<a href=\"/html/{}\">Next</a>", next_state.to_string());

    html += "<pre style=\"font-family: monospace\">\n";
    let bounds = next_state.bounds();
    println!("{:?}", bounds);
    for row in bounds.min_row-1..bounds.max_row+2 {
        for col in bounds.min_col-1..bounds.max_col+2 {
            html += if next_state.coord_alive(Coord { row, col }) {
                "*"
            } else {
                "."
            };
        }
        html += "\n";
    }
    html += "</pre>";

    html += "</body>";

    Ok(Html(html))
}
