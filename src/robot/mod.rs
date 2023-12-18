use std::{
    io::Read,
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use serde_derive::{Deserialize, Serialize};
use serde_json::{json, Number, Value};

use crate::{
    send_and_receive_data,
    server::{self, Color, Grid, Position, PositionWithColor},
    Queue,
};

const MY_IP: &str = "PLACE HOLDER";

pub fn create_stream() {}

pub fn robot_read(
    stream: Arc<Mutex<TcpStream>>,
    sort_request: Arc<Mutex<bool>>,
    finished_orders: Arc<Mutex<Queue<(Vec<Position>, u16)>>>,
    grid: Arc<Mutex<Grid>>,
) {
    loop {
        let mut buffer = String::new();
        {
            let mut read_half = stream.lock().unwrap();

            read_half.read_to_string(&mut buffer);
        }

        let finished_orders = finished_orders.clone();
        let sort_request = Arc::clone(&sort_request);
        let grid = Arc::clone(&grid);
        if buffer.len() > 0 {
            thread::spawn(move || {
                interpret_robot(buffer, finished_orders, sort_request, grid);
            });
        }
    }
}

fn interpret_robot(
    buffer: String,
    finished_orders: Arc<Mutex<Queue<(Vec<Position>, u16)>>>,
    sort_request: Arc<Mutex<bool>>,
    grid: Arc<Mutex<Grid>>,
) {
    if buffer.len() > 0 {
        let s: Value = serde_json::from_str(&buffer).unwrap();

        let command_type = s["command"].to_string();
        let command_type = command_type.as_str();

        match command_type {
            "order_confirm" => {
                let order_id = s["order_id"].as_u64().unwrap();
                let positions: Vec<Position> =
                    serde_json::from_str(&s["positions"].to_string()).unwrap();
                finished_orders
                    .lock()
                    .unwrap()
                    .list_of_queue
                    .push_front((positions, order_id as u16));
            }
            "sort_confirm" => {
                let x = s["x"].as_u64().unwrap() as u8;
                let y = s["y"].as_u64().unwrap() as u8;
                let color = s["color"].as_u64().unwrap() as u8;
                let color = Color::from(color);
                grid.lock().unwrap().sort_insert_lager_position(x, y, color);
            }
            "sort_request" => {
                *sort_request.lock().unwrap() = true;
                let pos = grid.lock().unwrap().get_free_position().unwrap();
                send_sort(Position {
                    position_x: pos.0,
                    position_y: pos.1,
                });
            }
            _ => {}
        }
    }
}

pub fn send_order(order_id: u8, positions: Vec<Position>) {
    let person_json = json!({
        "command": "order",
        "order-id": order_id,
        "positions": positions
    });
    send_and_receive_data("dd", person_json.to_string().as_str());
}

fn send_sort(position: Position) {
    let person_json = json!({
        "command": "sort_info",
        "x": position.position_x,
        "y": position.position_y,
    });
    send_and_receive_data("dd", person_json.as_str().unwrap());
}
