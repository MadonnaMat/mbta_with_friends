use crate::schema::stops;
use crate::schema::sublines;
use crate::schema::subline_stops;
use crate::schema::lines;

#[derive(Queryable, QueryableByName, Serialize)]
#[table_name="stops"]
pub struct Stop {
    pub id: i32,
    pub name: String,
    pub api_id: String,
    pub longitude: f64,
    pub latitude: f64,
}

#[derive(Queryable, QueryableByName, Serialize, Clone)]
#[table_name="sublines"]
pub struct Subline {
    pub id: i32,
    pub line_id: i32,
    pub name: String,
    pub api_id: String,
}

#[derive(Queryable, QueryableByName, Serialize, Clone)]
#[table_name="subline_stops"]
pub struct SublineStop {
    pub id: i32,
    pub subline_id: i32,
    pub stop_id: i32,
    pub stop_sequence: i32,
}

#[derive(Queryable, QueryableByName, Serialize, Clone)]
#[table_name="lines"]
pub struct Line {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize)]
pub struct JsonStop {
    pub id: i32,
    pub name: String,
    pub api_id: String,
    pub longitude: f64,
    pub latitude: f64,
    pub lines: Vec<JsonLine>,
}

#[derive(Serialize)]
pub struct JsonLine {
    pub id: i32,
    pub stop_sequence: i32,
    pub name: String,
    pub subline_name: String,
    pub api_id: String,
}

impl Stop {
    pub fn to_json(&self, d_lines: &Vec<Line>, d_sublines: &Vec<Subline>, d_subline_stops: &Vec<SublineStop>) -> JsonStop {
        let d_subline_stops = d_subline_stops
            .into_iter()
            .filter(|subs| subs.stop_id == self.id)
            .cloned()
            .collect();

        let json_line = d_sublines.iter()
            .map(|sub| sub.to_json(&d_lines, &d_subline_stops))
            .collect();

        JsonStop {
            id: self.id,
            name: String::from(&self.name[..]),
            api_id: String::from(&self.api_id[..]),
            longitude: self.longitude,
            latitude: self.latitude,
            lines: json_line
        }
    }
}

impl Subline {
    pub fn to_json(&self, d_lines: &Vec<Line>, d_subline_stops: &Vec<SublineStop>) -> JsonLine {
        let d_line = d_lines
            .into_iter()
            .filter(|line| line.id == self.line_id)
            .cloned()
            .collect::<Vec<Line>>();

        let d_line = d_line
            .first()
            .unwrap();

        let d_subline_stop =  d_subline_stops
            .into_iter()
            .filter(|line| line.subline_id == self.id)
            .cloned()
            .collect::<Vec<SublineStop>>();

        let d_subline_stop = d_subline_stop
            .first()
            .unwrap();

        JsonLine {
            id: self.id,
            stop_sequence: d_subline_stop.stop_sequence,
            name: String::from(&d_line.name[..]),
            subline_name: String::from(&self.name[..]),
            api_id: String::from(&self.api_id[..]),
        }
    }
}
