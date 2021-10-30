use life::{Coord, LifeState};

#[macro_use]
extern crate rocket;

mod life;

use rocket::response::content::Html;
use rocket::response::status::BadRequest;

// entry point
#[launch]
fn rocket() -> _ {
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .unwrap();

    rocket::custom(rocket::Config::figment()
        .merge(("port", port))
        .merge(("address", "0.0.0.0")))
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

    html += STYLES;

    html += LINKS;

    html += &format!("<a href=\"/html/{}\">Next</a>", next_state.to_string());

    html += "<table style=\"font-family: monospace\">\n";
    let bounds = next_state.bounds();
    println!("{:?}", bounds);
    for row in bounds.min_row-1..bounds.max_row+2 {
        html += "<tr>";
        for col in bounds.min_col-1..bounds.max_col+2 {
            let background = if next_state.coord_alive(Coord { row, col }) {
                "black"
            } else {
                "white"
            };

            html += &format!("<td style=\"background:{};\"></td>", background);
        }
        html += "</tr>";
    }
    html += "</table>";

    html += "</body>";

    Ok(Html(html))
}

static LINKS: &str = "
    <div>
        Links:
        <a href=\"/html/-1x0~0x0~1x0\">Oscillator</a>
        <a href=\"/html/0x1~1x2~2x0~2x1~2x2\">Glider</a>
        <a href=\"/html/0x38~1x38~1x39~1x40~1x41~2x32~2x33~2x39~2x40~2x41~2x42~2x50~3x27~3x31~3x34~3x39~3x42~3x49~3x51~4x26~4x28~4x32~4x33~4x39~4x40~4x41~4x42~4x47~4x48~4x52~5x9~5x10~5x26~5x27~5x29~5x38~5x39~5x40~5x41~5x47~5x48~5x52~5x62~5x63~6x9~6x11~6x26~6x27~6x29~6x30~6x38~6x47~6x48~6x52~6x62~6x63~7x4~7x5~7x12~7x26~7x27~7x29~7x49~7x51~8x0~8x1~8x3~8x6~8x9~8x12~8x26~8x28~8x50~9x0~9x1~9x4~9x5~9x12~9x21~9x27~9x39~9x41~10x9~10x11~10x19~10x21~10x39~10x40~11x9~11x10~11x20~11x21~11x40~14x43~14x44~15x43~15x44~22x40~22x41~23x40~24x41~24x42~24x43~25x43\">Glider gun</a>
    </div>
";

static STYLES: &str = "
<style>
    table {
        margin-top:20px;
        border-collapse: collapse;
    }
    td {
        width:20px;
        height:20px;
        border: 1px solid black;
        padding:0;
    }
</style>
";