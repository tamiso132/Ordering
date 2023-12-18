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

#[derive(Serialize, Deserialize)]
enum SendType {
    Order, // send order info to robot
    Sort,  // send sort info to robot
}

#[derive(Serialize, Deserialize)]
#[repr(u8)]
enum Receivetype {
    OrderConfirmation, // the order is done
    SortConfirmation,  // confirm I put into lager
    SortRequest,       // where to place
}

impl From<u8> for Receivetype {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::OrderConfirmation,
            1 => Self::SortConfirmation,
            2 => Self::SortRequest,
            _ => panic!("Command is not a thing"),
        }
    }
}

pub fn create_stream() {}

pub fn robot_read(
    stream: Arc<Mutex<TcpStream>>,
    sort_request: Arc<Mutex<bool>>,
    finished_orders: Arc<Mutex<Queue<(Vec<PositionWithColor>, u16)>>>,
) {
    loop {
        let mut buffer = String::new();
        {
            let mut read_half = stream.lock().unwrap();

            read_half.read_to_string(&mut buffer);
        }

        if buffer.len() > 0 {
            let finished_orders = finished_orders.clone();
            thread::spawn(move || {
                interpret_robot(buffer, finished_orders, sort_request);
            });
        }
    }
}

fn write_robot() {}

fn interpret_robot(
    buffer: String,
    finished_orders: Arc<Mutex<Queue<(Vec<PositionWithColor>, u16)>>>,
    sort_request: Arc<Mutex<bool>>,
) {
    if buffer.len() > 0 {
        let s: Value = serde_json::from_str(&buffer).unwrap();

        let command_type = s["command"].to_string().as_str();

        match command_type {
            "order_confirm" => {
                let order_id = s["order_id"].as_u64().unwrap();
                let positions: Vec<PositionWithColor> =
                    serde_json::from_str(&s["positions"].to_string()).unwrap();
                finished_orders
                    .lock()
                    .unwrap()
                    .list_of_queue
                    .push_front((order_id, positions));

                // TODO send confirm to database
            }
            "sort_confirm" => {
                //TODO Update database
                server::send_order_done_db(positions, order_id)
            }
            "sort_request" => {
                *sort_request.lock().unwrap() = true;
            }
        }
    }
}

pub fn send_order(order_id: u8, positions: Vec<Position>) {
    let person_json = json!({
        "command": "order",
        "order id": order_id,
        "positions": positions
    });
    send_and_receive_data("dd", person_json.to_string().as_str());
}

fn send_sort(position: Position, color: Color) {
    let person_json = json!({
        "command": "sort_info",
        "color": color as u8,
        "x": position.position_x,
        "y": position.position_y,
    });
    send_and_receive_data("dd", person_json.as_str().unwrap());
}

fn receive_orderconfrimation(order_id: u8) {
    let person_json = json!({
      "command": Receivetype::OrderConfirmation as u8,
      //"positions" vec<Positions>
      "order_id": order_id,
    });
}

fn receive_sortconfrimation() {
    let person_json = json!({
     "command": Receivetype::SortConfirmation as u8,
    });
}

fn received_sortrequest() {
    let oerson_json = json!({
        "command":Receivetype::SortRequest as u8
    });
}
