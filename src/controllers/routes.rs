use rocket::response::status::NotFound;
use rocket_contrib::Json;

use crate::pg_pool::DbConn;
use crate::models::user::User;
use crate::models::stop::*;

use diesel::RunQueryDsl;

use crate::schema::stops::dsl::*;
use crate::schema::lines::dsl::*;
use crate::schema::sublines::dsl::*;
use crate::schema::subline_stops::dsl::*;

#[get("/routes", rank=1)]
pub fn all(conn: DbConn, current_user: User) -> Result<Json<Vec<JsonStop>>, NotFound<String>> {
    let d_stops = match stops.load::<Stop>(&*conn) {
        Ok(d_stops) => d_stops,
        Err(e) => return Err(NotFound(format!("{}", e)))
    };

    let d_lines = match lines.load::<Line>(&*conn) {
        Ok(d_lines) => d_lines,
        Err(e) => return Err(NotFound(format!("{}", e)))
    };

    let d_sublines = match sublines.load::<Subline>(&*conn) {
        Ok(d_sublines) => d_sublines,
        Err(e) => return Err(NotFound(format!("{}", e)))
    };

    let d_subline_stops = match subline_stops.load::<SublineStop>(&*conn){
        Ok(d_subline_stops) => d_subline_stops,
        Err(e) => return Err(NotFound(format!("{}", e)))
    };

    Ok(Json(d_stops.iter().map(|stp| stp.to_json(&d_lines, &d_sublines, &d_subline_stops)).collect()))
}
