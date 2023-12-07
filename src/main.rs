use std::{
    collections::VecDeque,
    io::{Read, Write},
    net::{SocketAddr, TcpStream},
    sync::{Arc, Mutex},
    thread,
};

use server::Grid;

// Importera nödvändiga bibliotek för serialisering och deserialiserinf av JSON

mod robot;
mod server;

// process, läsa om ny order har kommit,
// proccess, läsa info om robot
// main process,

pub fn send_and_receive_data(ip: &str, data: &str) -> Result<String, std::io::Error> {
    // Parse the IP address
    let addr: SocketAddr = format!("{}:8080", ip).parse().unwrap();

    // Elablera en TCP-anslutning
    let mut stream = TcpStream::connect(addr)?;

    // Skicka data till servern
    stream.write_all(data.as_bytes())?;

    // Läs svaret från servern
    let mut buffer = String::new();
    stream.read_to_string(&mut buffer)?;

    Ok(buffer)
}

fn main() {
    let grid = Arc::new(Mutex::new(Grid::new()));

    let database_grid = grid.clone();
    thread::spawn(move || {
        read_order_from_database();
    });
}

fn read_order_from_database() {
    // läser från databasen för en order

    // let mut stream = TcpStream::connect("127.0.0.1:34254")?;

    // stream.write(&[1])?;
    // stream.read(&mut [0; 128])?;

    // gör om json format till class
    // let example_json = "example";

    // let order = serde_json::from_str::<Order>(example_json).unwrap();

    // let mut blue = 0; // antal blå i beställning
    // let mut red = 0;
    // let mut green = 0;
    // let mut yellow = 0;

    // for e in order.amount {
    //     let amount = e.total_product_amount; // antal av den färgen
    //     let product_type = e.product_type; // vilken typ av färg

    //     match product_type.as_str() {
    //         "Red block" => red += amount,
    //         "Yellow block" => yellow += amount,
    //         "Green block" => green += amount,
    //         "Blue block" => blue += amount,
    //         _ => {
    //             panic!("FICK KONSTIG info från database {}", product_type) // databasen skickar fel info
    //         }
    //     }
    // }
    // hitta positioner för de olika färgerna
}

fn send_order() {}

fn send_robot() {
    // läs från databasen
    // om de finns beställning
    // skicka Order till robot
}

fn interpret_robot() {
    // let order_type = match typ {
    //     0 => {
    //         // skicka till servern att beställning är klar
    //         // ta bort den från listan
    //     }
    //     1 => {
    //         // uppdatera listan
    //         // skicka uppdatering till servern
    //     }
    //     2 => {
    //         // hitta en ledig position,
    //         // skicka den lediga platsen till robotsystemet
    //     }
    // };
}
