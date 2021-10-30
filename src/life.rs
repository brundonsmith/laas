use std::collections::HashMap;
use rocket::{http::Header, request::FromParam, response::Responder};

static AXIS_DELIM: &str = "x";
static COORD_DELIM: &str = "~";

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Coord {
    pub row: i64,
    pub col: i64,
}

impl Coord {
    pub fn neighbors(&self) -> [Coord; 8] {
        [
            Coord { row: self.row + 1, col: self.col + 1 },
            Coord { row: self.row + 1, col: self.col - 1 },
            Coord { row: self.row - 1, col: self.col + 1 },
            Coord { row: self.row - 1, col: self.col - 1 },
            Coord { row: self.row + 1, col: self.col },
            Coord { row: self.row,     col: self.col + 1 },
            Coord { row: self.row - 1, col: self.col },
            Coord { row: self.row,     col: self.col - 1 },
        ]
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Bounds {
    pub min_row: i64,
    pub min_col: i64,
    pub max_row: i64,
    pub max_col: i64,
}

#[derive(Debug)]
pub struct LifeState(HashMap<Coord, bool>);


// Rocket traits
impl<'a> FromParam<'a> for LifeState {
    type Error = &'static str;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        param
            .split(COORD_DELIM)
            .map(|coord_str| {
                let mut nums = coord_str.split(AXIS_DELIM).map(|n| n.parse::<i64>());

                let row = nums.next().ok_or(())?.map_err(|_| ())?;
                let col = nums.next().ok_or(())?.map_err(|_| ())?;

                Ok((Coord { row, col }, true))
            })
            .collect::<Result<HashMap<Coord, bool>, ()>>()
            .map(|set| LifeState(set))
            .map_err(|_| "Failed to parse input string")
    }
}

impl<'r,'o: 'r> Responder<'r,'o> for LifeState {
    fn respond_to(self, req: &'r rocket::Request<'_>) -> rocket::response::Result<'o> {
        self.to_string().respond_to(req).map(|mut res| {
            res.set_header(Header::new("Access-Control-Allow-Origin", "*"));
            res
        })
    }
}


// Core logic
impl LifeState {
    
    pub fn to_string(&self) -> String {
        let mut segments: Vec<String> = self
            .0
            .iter()
            .filter(|(_, alive)| **alive)
            .map(|(coord, _)| coord.row.to_string() + AXIS_DELIM + &coord.col.to_string())
            .collect();
            
        segments.sort();

        segments.join(COORD_DELIM)
    }

    pub fn next_state(self) -> Self {
        let mut new_map = HashMap::new();

        let coords_to_check = self.0.keys().map(|known| std::iter::once(*known).chain(known.neighbors().into_iter())).flatten();

        for coord in coords_to_check {
            if !new_map.contains_key(&coord) {
                let alive = self.coord_alive(coord);
                let alive_neighbors = coord.neighbors().iter().filter(|neighbor| *self.0.get(*neighbor).unwrap_or(&false)).count();

                new_map.insert(coord, new_status(alive, alive_neighbors));
            }
        }

        LifeState(new_map)
    }

    pub fn bounds(&self) -> Bounds {
        let mut bounds = None;

        for coord in self.0.iter().filter(|(_, alive)| **alive).map(|(coord, _)| coord) {
            if bounds == None {
                bounds = Some(Bounds {
                    min_row: coord.row,
                    min_col: coord.col,
                    max_row: coord.row,
                    max_col: coord.col,
                });
            } else if let Some(bounds_contents) = bounds {
                bounds = Some(Bounds {
                    min_row: i64::min(bounds_contents.min_row, coord.row),
                    min_col: i64::min(bounds_contents.min_col, coord.col),
                    max_row: i64::max(bounds_contents.max_row, coord.row),
                    max_col: i64::max(bounds_contents.max_col, coord.col),
                });
            }
        }

        bounds.unwrap()
    }

    pub fn coord_alive(&self, coord: Coord) -> bool {
        self.0.get(&coord).map(|b| *b).unwrap_or(false)
    }
}

fn new_status(alive: bool, alive_neighbors: usize) -> bool {
    if alive && (alive_neighbors == 2 || alive_neighbors == 3) {
        true
    } else if !alive && alive_neighbors == 3 {
        true
    } else {
        false
    }
}
