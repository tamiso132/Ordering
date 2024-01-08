use std::{
    io::{Read, Write},
    net::TcpStream,
    sync::{Arc, Mutex},
    time::Duration,
};

use serde_json::{json, Value};

use crate::{
    server::{self, Color, Grid, Position},
    write_log_file, Queue, Thing,
};

const MY_IP: &str = "192.168.88.71:7070";

pub fn robot_read(thing: Arc<Mutex<Thing>>, stream: Arc<Mutex<TcpStream>>) {
    let mut buffer = String::new();
    let s_clone = stream.clone();
    stream.lock().unwrap().read_to_string(&mut buffer);
    if buffer.len() > 0 {
        interpret_robot(thing, buffer, s_clone);
    }
}

fn interpret_robot(mut thing: Arc<Mutex<Thing>>, buffer: String, stream: Arc<Mutex<TcpStream>>) {
    if buffer.len() > 0 {
        println!("buffer: {}", buffer);
        let s: Value = serde_json::from_str(&buffer).unwrap();

        let command_type = s["command"].to_string();
        let command_type = command_type.as_str().trim();

        println!("{}", command_type);

        if command_type.contains("order_confirm") {
            let order_id = s["order_id"].as_u64().unwrap();
            let positions: Vec<Position> =
                serde_json::from_str(&s["positions"].to_string()).unwrap();
            println!("Order confirm, positions");
            thing
                .lock()
                .unwrap()
                .finished_orders
                .list_of_queue
                .push_front((positions, order_id as u16));
            write_log_file("Order completed, received from robot");
            println!("added to finished orders");
        }
        if command_type.contains("sort_confirm") {
            let x = s["x"].as_u64().unwrap() as u8;
            let y = s["y"].as_u64().unwrap() as u8;
            let color = s["color"].as_u64().unwrap() as u8;
            let color = Color::from(color);
            thing
                .lock()
                .unwrap()
                .grid
                .sort_insert_lager_position(x, y, color);
            write_log_file("Sort completed, received from robot");
        }
        if command_type.contains("sort_request") {
            thing.lock().unwrap().sort_request = true;
            println!("I get sort request, {}", s);
            let color = s["color"].to_string().parse::<u64>().unwrap();
            let pos = thing.lock().unwrap().grid.get_free_position().unwrap();
            write_log_file("Sort request, received from robot");
            send_sort(
                Position {
                    position_x: pos.0,
                    position_y: pos.1,
                },
                stream,
                color,
            );
        }
    }
}

pub fn send_order(order_id: u8, positions: Vec<Position>, stream: Arc<Mutex<TcpStream>>) {
    let person_json = json!({
        "command": "order",
        "order-id": order_id,
        "positions": positions
    });

    println!("order sent: {}", person_json);
    stream
        .lock()
        .unwrap()
        .write(person_json.to_string().as_bytes())
        .unwrap();
}

fn send_sort(position: Position, stream: Arc<Mutex<TcpStream>>, color: u64) {
    let person_json = json!({
        "command": "sort_info",
        "color": color,
        "x": position.position_x,
        "y": position.position_y,
    });
    println!("sort request {}", person_json);
    stream
        .lock()
        .unwrap()
        .write_all(person_json.to_string().as_bytes())
        .unwrap();
}

pub fn send_start(stream: Arc<Mutex<TcpStream>>) {
    let json = json!({
        "command": "start"
    });
    stream
        .lock()
        .unwrap()
        .write_all(json.to_string().as_bytes())
        .unwrap();
}

pub fn send_stop(stream: Arc<Mutex<TcpStream>>) {
    let json = json!({
        "command": "stop"
    });
    stream
        .lock()
        .unwrap()
        .write_all(json.to_string().as_bytes())
        .unwrap();
}
