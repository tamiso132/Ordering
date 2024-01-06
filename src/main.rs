use std::{
    collections::VecDeque,
    io::{Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    ops::Deref,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use eframe::glow::CONSTANT_COLOR;
use serde_derive::{Deserialize, Serialize};
use server::{Grid, Position};

// Importera nödvändiga bibliotek för serialisering och deserialiserinf av JSON

mod graphic;
mod robot;
pub mod server;

// process, läsa om ny order har kommit,
// proccess, läsa info om robot
// main process,

pub fn send_and_receive_data(ip: &str, data: &str) -> Result<String, std::io::Error> {
    // Parse the IP address

    let addr: SocketAddr = format!("{}", ip).parse().unwrap();
    // Elablera en TCP-anslutning
    let mut stream = TcpStream::connect(addr).unwrap();

    // Skicka data till servern
    stream.write_all(data.as_bytes()).unwrap();

    // Läs svaret från servern
    let mut buffer = String::new();
    stream.read_to_string(&mut buffer).unwrap();

    Ok(buffer)
}

#[derive(Clone, Serialize, Deserialize)]
struct Queue<T: Clone>
// En generisk köstruktur som används för att hantera olika typer av order.
where
    T: Clone,
{
    list_of_queue: VecDeque<T>,
}

// En trådfunktion som periodiskt läser uppdateringar från en databas och lägger dem i kön för att processas.
fn read_database_thread(thing: Arc<Mutex<Thing>>) {
    // if !thing.lock().unwrap().is_order_in_process.clone() {
    match server::read_order_updates() {
        Some(order) => {
            thing
                .lock()
                .unwrap()
                .orders_to_process
                .list_of_queue
                .push_front(order);

            thing.lock().unwrap().is_order_in_process = true;
        }
        None => {}
    }
    //  }
}

#[derive(Serialize, Deserialize)]
pub struct Thing {
    grid: Grid,
    current_order: Option<([u16; 4], u16)>,
    finished_orders: Queue<(Vec<Position>, u16)>,
    orders_to_process: Queue<([u16; 4], u16)>,
    history_orders: Vec<([u16; 4], u16)>,

    is_order_in_process: bool,
    sort_request: bool,
}

impl Thing {
    // stream: Arc<Mutex<TcpStream>>
    pub fn run(mutex: Arc<Mutex<Self>>) {
        loop {
            thread::sleep(Duration::from_millis(200));
            // robot::robot_read(mutex.clone(), stream.clone());
            thread::sleep(Duration::from_millis(200));
            read_database_thread(mutex.clone()); // PUT TIMERS ON STUFF, TO GIVE
            thread::sleep(Duration::from_millis(200));
            process_order_queue(mutex.clone());
            thread::sleep(Duration::from_millis(200));
            process_finished_order(mutex.clone());
            thread::sleep(Duration::from_millis(200));
        }
    }
}

const MY_IP: &str = "192.168.43.45:7071"; // Servens IP-adress och portnummer.
fn main() {
    let grid = Grid::new();
    // let stream_ = TcpStream::connect("192.168.88.222:12000").unwrap();

    // stream_.set_nonblocking(true);
    // stream_.set_read_timeout(Some(Duration::from_millis(100)));

    //  let stream = Arc::new(Mutex::new(stream_));

    let is_order_in_process = false;
    let current_order: Option<([u16; 4], u16)> = None;
    let finished_orders: Queue<(Vec<Position>, u16)> = Queue {
        list_of_queue: VecDeque::new(),
    };
    let mut orders_to_process: Queue<([u16; 4], u16)> = Queue {
        list_of_queue: VecDeque::new(),
    };

    let history_orders: Vec<([u16; 4], u16)> = vec![];

    let sort_request = false;

    // TODO TEST CODE, REMEMBER TO DELETE IT

    // orders_to_process
    //     .list_of_queue
    //     .push_front(([3, 0, 0, 0], 1));

    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
                        //let current_orders_1 = Arc::clone(&orders_to_process);
    let all = Thing {
        grid,
        current_order,
        finished_orders,
        orders_to_process,
        is_order_in_process,
        sort_request,
        history_orders,
    };
    let mutex = Arc::new(Mutex::new(all));
    let second = mutex.clone();
    //  let second_stream = stream.clone();

    thread::sleep(Duration::from_millis(100));
    std::thread::spawn(|| Thing::run(mutex));

    loop {
        let second = second.clone();
        // let second_stream = second_stream.clone();
        graphic::run(second).unwrap();
    }
}

fn process_finished_order(thing: Arc<Mutex<Thing>>) {
    let orders_done_len = thing
        .lock()
        .unwrap()
        .finished_orders
        .list_of_queue
        .len()
        .clone();
    if orders_done_len > 0 {
        let order_done = thing
            .lock()
            .unwrap()
            .finished_orders
            .list_of_queue
            .pop_front()
            .unwrap();

        let d = thing.lock().unwrap().current_order.clone().unwrap();
        thing.lock().unwrap().history_orders.push((d.0, d.1));
        thing.lock().unwrap().current_order = None;
        server::send_order_done_db(order_done.0, order_done.1 as u32);
    }
}

fn process_order_queue(thing: Arc<Mutex<Thing>>) {
    match thing.lock() {
        Ok(mut thing) => {
            if !thing.is_order_in_process {
                if thing.orders_to_process.list_of_queue.len() > 0 {
                    thing.is_order_in_process = true;
                    let order_to_send = thing
                        .orders_to_process
                        .list_of_queue
                        .pop_front()
                        .unwrap()
                        .clone();
                    thing.current_order = Some(order_to_send);
                    let positions = thing.grid.get_positions_for_order(order_to_send.0);

                    // robot::send_order(order_to_send.1 as u8, positions, stream.clone());
                }
            }
        }
        Err(e) => println!("{e}"),
    }
}
