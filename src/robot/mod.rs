use std::{
    io::{Read, Write},
    net::TcpStream,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use serde_json::{json, Value};

use crate::{
    server::{self, Color, Grid, Position},
    Queue,
};

const MY_IP: &str = "192.168.88.71:7070";

pub fn robot_read(
    mut stream: Arc<Mutex<TcpStream>>,
    sort_request: Arc<Mutex<bool>>,
    finished_orders: Arc<Mutex<Queue<(Vec<Position>, u16)>>>,
    grid: Arc<Mutex<Grid>>,
) {
    loop {
        let mut buffer = String::new();
        stream.lock().unwrap().read_to_string(&mut buffer);
        if buffer.len() > 0 {
            let finished_orders = finished_orders.clone();
            let sort_request = Arc::clone(&sort_request);
            let grid = Arc::clone(&grid);
            println!("does it even come here?,, blu");
            interpret_robot(
                Arc::clone(&stream),
                buffer,
                finished_orders,
                sort_request,
                grid,
            );
        }
    }
}

fn interpret_robot(
    stream: Arc<Mutex<TcpStream>>,
    buffer: String,
    finished_orders: Arc<Mutex<Queue<(Vec<Position>, u16)>>>,
    sort_request: Arc<Mutex<bool>>,
    grid: Arc<Mutex<Grid>>,
) {
    if buffer.len() > 0 {
        let s: Value = serde_json::from_str(&buffer).unwrap();

        let command_type = s["command"].to_string();
        let command_type = command_type.as_str().trim();

        println!("{}", command_type);

        if command_type.contains("order_confirm") {
            let order_id = s["order_id"].as_u64().unwrap();
            let positions: Vec<Position> =
                serde_json::from_str(&s["positions"].to_string()).unwrap();
            finished_orders
                .lock()
                .unwrap()
                .list_of_queue
                .push_front((positions, order_id as u16));
        }
        if command_type.contains("sort_confirm") {
            let x = s["x"].as_u64().unwrap() as u8;
            let y = s["y"].as_u64().unwrap() as u8;
            let color = s["color"].as_u64().unwrap() as u8;
            let color = Color::from(color);
            grid.lock().unwrap().sort_insert_lager_position(x, y, color);
        }
        if command_type.contains("sort_request") {
            println!("I get sort request");
            *sort_request.lock().unwrap() = true;
            let color = s["color"].to_string().parse::<u64>().unwrap();
            let pos = grid.lock().unwrap().get_free_position().unwrap();
            send_sort(
                Position {
                    position_x: pos.0,
                    position_y: pos.1,
                },
                stream,
                color,
            );
            println!("does it come here");
        }
    }
}

pub fn send_order(order_id: u8, positions: Vec<Position>, stream: Arc<Mutex<TcpStream>>) {
    let person_json = json!({
        "command": "order",
        "order-id": order_id,
        "positions": positions
    });
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
