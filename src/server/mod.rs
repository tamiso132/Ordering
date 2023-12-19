use serde_derive::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    send_and_receive_data,
    server::internal::{get_order_from_db, request, SERVER_IP},
};

use self::internal::{get_positions_from_db, update_position};

mod internal;

const XMAX: usize = 4;
const YMAX: usize = 6;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Position {
    pub position_x: usize,
    pub position_y: usize,
}
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct PositionWithColor {
    pub position_x: usize,
    pub position_y: usize,
    pub product_type_id: u8,
}

#[repr(u8)]
#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Color {
    Red,
    Yellow,
    Green,
    Blue,
}
impl Color {
    pub fn from_str(s: &str) -> Option<Color> {
        match s {
            "red" => Some(Color::Red),
            "yellow" => Some(Color::Yellow),
            "green" => Some(Color::Green),
            "blue" => Some(Color::Blue),
            _ => None,
        }
    }

    pub fn to_str(self) -> &'static str {
        match self {
            Color::Red => "red",
            Color::Yellow => "yellow",
            Color::Green => "green",
            Color::Blue => "blue",
            _ => panic!("unknown"),
        }
    }
}
impl From<u8> for Color {
    fn from(value: u8) -> Self {
        match value {
            0 => Color::Red,
            1 => Color::Yellow,
            2 => Color::Green,
            3 => Color::Blue,
            _ => {
                panic!("not existing color")
            }
        }
    }
}

pub struct Grid {
    grid: [[i8; XMAX]; YMAX],
}
impl Grid {
    pub fn new() -> Self {
        //TODO get init from database
        let grid = get_positions_from_db();

        Self { grid }
    }
    pub fn get_positions_for_order(&self, mut objects: [u16; 4]) -> Vec<Position> {
        let mut positions = vec![];
        for y in 0..YMAX {
            for x in 0..XMAX {
                let color = self.grid[x][y];
                match color {
                    -1 => {}
                    0 => {
                        if objects[Color::Red as usize] > 0 {
                            objects[Color::Red as usize] -= 1;
                            positions.push(Position {
                                position_x: x,
                                position_y: y,
                            });
                        }
                    }
                    1 => {
                        if objects[Color::Yellow as usize] > 0 {
                            objects[Color::Yellow as usize] -= 1;
                            positions.push(Position {
                                position_x: x,
                                position_y: y,
                            });
                        }
                    }
                    2 => {
                        if objects[Color::Green as usize] > 0 {
                            objects[Color::Green as usize] -= 1;
                            positions.push(Position {
                                position_x: x,
                                position_y: y,
                            });
                        }
                    }
                    3 => {
                        if objects[Color::Blue as usize] > 0 {
                            objects[Color::Blue as usize] -= 1;
                            positions.push(Position {
                                position_x: x,
                                position_y: y,
                            });
                        }
                    }
                    _ => {}
                }
            }
        }
        positions
    }
    pub fn sort_insert_lager_position(&mut self, x: u8, y: u8, color: Color) {
        self.grid[x as usize][y as usize] = color as i8;

        update_position(x as u32, y as u32, color);
    }

    pub fn get_free_position(&self) -> Option<(usize, usize)> {
        for y in 0..YMAX {
            for x in 0..XMAX {
                if self.grid[x][y] == -1 {
                    return Some((x, y));
                }
            }
        }
        None
    }
}

#[derive(Serialize, Deserialize)]
struct OrderSend {
    position_x: usize,
    position_y: usize,
    product_type_id: i64,
}

pub fn send_order_done_db(positions: Vec<Position>, order_id: u32) {
    let mut order_send = vec![];
    for p in positions {
        let position_x = p.position_x;
        let position_y = p.position_y;
        let product_type_id = -1;
        order_send.push(OrderSend {
            position_x,
            position_y,
            product_type_id,
        });
    }
    let data_to_send = json!({"updated_positions": order_send});

    let full_request = format!(
        "REPORT orders/{} ORDSYS/1.0\nStatus: OK\n{}",
        order_id, data_to_send
    );

    send_and_receive_data(SERVER_IP, &full_request);
    println!("{}", full_request);
}

pub fn read_order_updates() -> Option<([u16; 4], u16)> {
    let order_json = get_order_from_db();

    if order_json == None {
        println!("No Order");
        return None;
    }
    let order_json = order_json.unwrap();
    let mut order_id = 0;
    let mut total_amount = [0, 0, 0, 0];

    for line in order_json.lines() {
        if line.contains("\"id\"") {
            let number_str = line[6..line.len() - 1].trim();
            let id = number_str.parse::<u16>().unwrap();
            order_id = id;
        }

        if line.contains("\"product_type\"") {
            let str_total = "\"total_product_amount\":";
            let str_color = "{\"product_type\": \"";

            let color_end_index = line.find("block").unwrap() - 1;
            let color_str = &line[str_color.len()..color_end_index].to_lowercase();

            let color = Color::from_str(&color_str).unwrap();

            let product_total_start = line.find(str_total).unwrap();

            let product_total_end = line.find("}").unwrap();

            let total_product_str = line
                [product_total_start + "\"total_product_amount\": ".len()..product_total_end]
                .trim();

            let amount = total_product_str.parse::<u16>().unwrap();

            total_amount[color as usize] += amount;
        }
    }

    Some((total_amount, order_id))

    // TODO check if any new orders
    // if any new orders, add them to a order queue
}
