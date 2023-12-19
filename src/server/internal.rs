use std::{
    io::{self, Write},
    net::TcpStream,
    ops::Add,
    sync::{Arc, Mutex},
};

use serde_derive::{Deserialize, Serialize}; // Importera attribut för att automatisk serialisering och deserialisering av strukturer med serde.
use serde_json::{json, Value}; // Importera funktioner och typer från Serde JSON för att arbeta med JSON-data.

use crate::send_and_receive_data;

use super::{Color, Grid, Position, XMAX, YMAX};

#[derive(Serialize, Deserialize)]
struct TempPos {
    position_x: u32,
    position_y: u32,
    product_type_id: u32,
}

pub(crate) mod request {
    pub(crate) const REQUEST_LINE_PROCESS: &'static str = "PROCESS orders/oldest ORDSYS/1.0";

    pub const REQUEST_UPDATE_POSITION: &'static str = "PATCH order-positions ORDSYS/1.0";

    pub(crate) const REQUEST_GET_ORDER_POSITIONS_FULL: &'static str =
        "GET order-positions ORDSYS/1.0";

    const REQUEST_LINE_REPORT: &'static str = "REPORT orders/id ORDSYS/1.0";

    const REQUEST_REPORT_OK_NO_BODY: &'static str = "REPORT orders/id ORDSYS/1.0\nStatus: OK";
    const REQUEST_LINE_REPORT_OK: &'static str = "REPORT orders/id ORDSYS/1.0";

    const REQUEST_REPORT_FAIL_NO_BODY: &'static str = "REPORT orders/id ORDSYS/1.0\nStatus: FAIL";
}

//pub const SERVER_IP: &'static str = "213.200.135.239:7878";
pub const SERVER_IP: &'static str = "192.168.88.71:7878";
const NO_ORDER: &str = "ORDSYS/1.0 NOT_READY";

pub fn order_confirm_db(positions: Vec<Position>) {}
//pub const SERVER_IP: &'static str = "213.200.135.239:7878";

pub fn get_order_from_db() -> Option<String> {
    let order_json = send_and_receive_data(SERVER_IP, request::REQUEST_LINE_PROCESS).unwrap();

    if order_json.contains(NO_ORDER) {
        // no order found
        println!("No Order");
        return None;
    }
    Some(order_json)
}

pub(crate) fn update_position(x: u32, y: u32, color: Color) {
    let mut temp = vec![];
    temp.push(TempPos {
        position_x: x as u32,
        position_y: y as u32,
        product_type_id: color as u32,
    });
    let json_data = json!({"updated_positions": temp});

    let output = format!("{}\n{}", request::REQUEST_UPDATE_POSITION, json_data);
    send_and_receive_data(SERVER_IP, &output);
}

pub(crate) fn get_positions_from_db() -> [[i8; XMAX]; YMAX] {
    let order_json =
        send_and_receive_data(SERVER_IP, request::REQUEST_GET_ORDER_POSITIONS_FULL).unwrap();

    let mut lines = order_json.lines();

    lines.next();
    lines.next();
    lines.next();

    println!("{}", order_json);

    let mut grid_info: [[i8; XMAX]; YMAX] = [[-1; XMAX]; YMAX];

    for line in lines {
        let lower_case = line.to_lowercase();

        let lower: Vec<_> = lower_case.split(",").collect();

        let position_x_str = " \"positions_x\": ";
        let position_y_str = " \"positions_y\": ";
        let empty_str = " \"empty\": ";
        let product_type_str = " \"product_type_id\": ";

        let mut position_x_val = 0;
        let mut position_y_val = 0;
        let mut empty_val = "0";
        let mut product_type = -1;

        for attribute in lower {
            if attribute.contains(position_x_str) {
                let val_str = &attribute[position_x_str.len()..attribute.len() - 1];
                position_x_val = val_str.parse::<u16>().unwrap();
            }
            if attribute.contains(position_y_str) {
                let val_str = &attribute[position_y_str.len()..attribute.len() - 1];
                position_y_val = val_str.parse::<u16>().unwrap();
            }
            if attribute.contains(empty_str) {
                let val_str = &attribute[empty_str.len()..attribute.len() - 1];
                empty_val = val_str;
            }
            if attribute.contains(product_type_str) {
                let val_str = &attribute[product_type_str.len()..attribute.len() - 1];
                product_type = val_str.parse::<i8>().unwrap();
            }
        }

        if empty_val.contains("true") {
            grid_info[position_x_val as usize][position_y_val as usize] = product_type;
        }
    }

    grid_info
}
