use std::{
    io::{self, Write},
    net::TcpStream,
    sync::{Arc, Mutex},
};

use serde_derive::{Deserialize, Serialize};
use serde_json::json;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env; 

use crate::{Position, Grid, SERVER_IP, send_and_receive_data};
 //use crate::send_and_receive_data;



#[repr(u8)]
enum Color {
    None,
    Red,
    Yellow,
    Green,
    Blue,
}

impl From<u8> for Color {
    fn from(value: u8) -> Self {
        match value {
            0 => Color::None,
            1 => Color::Red,
            2 => Color::Yellow,
            3 => Color::Green,
            4 => Color::Blue,
            _ => {
                panic!("not existing color")
            }
        }
    }
}

const XMAX: usize = 4;
const YMAX: usize = 6;

const AVAILABLE_ORDER_REQUEST: &'static str = "
PROCESS orders/oldest ORDSYS/1.0\n
{”status”: ”processing”}";
const LAGER_POSITIONS_REQUEST: &'static str = "PLACE HOLDER";

const SERVER_IP: &'static str = "PLACEHOLDER";

pub struct Grid {
    grid: [[u8; XMAX]; YMAX],
}
impl Grid {
    pub fn new() -> Self {
        //TODO get init from database
        let json_lager = send_and_receive_data(SERVER_IP, LAGER_POSITIONS_REQUEST).unwrap();

        Self {
            grid: [[0; XMAX]; YMAX],
        }
    }
    pub fn get_positions_for_order(
        &self,
        mut blue: u16,
        mut red: u16,
        mut yellow: u16,
        mut green: u16,
    ) -> Vec<Position> {
        let mut positions = vec![];
        for y in 0..YMAX {
            for x in 0..XMAX {
                let color = self.grid[x][y];
                match color {
                    0 => {}
                    1 => {
                        if red > 0 {
                            red -= 1;
                            positions.push(Position { x, y });
                        }
                    }
                    2 => {
                        if yellow > 0 {
                            yellow -= 1;
                            positions.push(Position { x, y });
                        }
                    }
                    3 => {
                        if green > 0 {
                            green -= 1;
                            positions.push(Position { x, y });
                        }
                    }
                    4 => {
                        if blue > 0 {
                            blue -= 1;
                            positions.push(Position { x, y });
                        }
                    }
                    _ => {}
                }
            }
        }
        positions
    }
    pub fn insert_lager_position(&mut self, x:u8, y:u8, color:Color){
        self.grid[x][y] = color;
    }
}

/// En struktur för att representera position (rad och kolumn)
#[derive(Debug, Serialize, Deserialize)]
pub struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    id: u32,
    amount: Vec<ProductOrder>,
}

impl Order {
    fn send_to_robot(&mut self, stream: &mut TcpStream, grid: Arc<Mutex<Grid>>) -> io::Result<()> {
        let mut blue = 0; // antal blå i beställning
        let mut red = 0;
        let mut green = 0;
        let mut yellow = 0;

        for e in &self.amount {
            let amount = e.total_product_amount; // antal av den färgen
            let product_type = &e.product_type; // vilken typ av färg

            match product_type.as_str() {
                "Red block" => red += amount,
                "Yellow block" => yellow += amount,
                "Green block" => green += amount,
                "Blue block" => blue += amount,
                _ => {
                    panic!("FICK KONSTIG info från database {}", product_type) // databasen skickar fel info
                }
            }
        }

        let positions;
        {
            let grid_locked = grid.lock().unwrap();
            positions = grid_locked.get_positions_for_order(blue, red, yellow, green);
        }
        let person_json = json!({
            "order id": self.id,
            "positions": positions
        });

        stream.write(person_json.to_string().as_bytes())?;
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ProductOrder {
    product_type: String,
    total_product_amount: u16,
}

pub fn read_order_updates() {
    let order_json = send_and_receive_data(SERVER_IP, LAGER_POSITIONS_REQUEST).unwrap();
    // TODO check if any new orders
    // if any new orders, add them to a order queue
}

