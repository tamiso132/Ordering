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
    server::{self, Color, Position},
    Queue,
};

const ROBOT_IP: &str = "PLACE HOLDER";

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

pub fn robot_read(finished_orders: Arc<Mutex<Queue<(Vec<Position>, u16)>>>) {
    let stream = TcpStream::connect(ROBOT_IP).unwrap();
    stream.set_read_timeout(Some(Duration::from_millis(500)));

    let mut stream_read = Arc::new(Mutex::new(stream));

    let mut stream_write = stream_read.clone();

    read_robot_commands(stream_read, finished_orders);
}

fn write_robot() {}

fn read_robot_commands(
    stream: Arc<Mutex<TcpStream>>,
    finished_orders: Arc<Mutex<Queue<(Vec<Position>, u16)>>>,
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
                interpret_robot(buffer, finished_orders);
            });
        }
    }
}

fn interpret_robot(buffer: String, finished_orders: Arc<Mutex<Queue<(Vec<Position>, u16)>>>) {
    if buffer.len() > 0 {
        let s: Value = serde_json::from_str(&buffer).unwrap();

        let command_type: Receivetype = (s["command"].as_u64().unwrap() as u8).into();

        match command_type {
            Receivetype::OrderConfirmation => {
                let order_id = s["order_id"].as_u64().unwrap();
                let positions: Vec<Position> =
                    serde_json::from_str(&s["positions"].to_string()).unwrap();

                // TODO send confirm to database
            }
            Receivetype::SortConfirmation => {
                //TODO Update database
            }
            Receivetype::SortRequest => todo!(),
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

fn send_sort(positions: Position, color: Color) {
    let person_json = json!({
        "command": SendType::Sort as u8,
        "color": color.to_str(),
        "positions": positions,
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
