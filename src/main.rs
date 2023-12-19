use std::{
    collections::VecDeque,
    io::{Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

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
    let mut stream = TcpStream::connect(addr)?;

    // Skicka data till servern
    stream.write_all(data.as_bytes())?;

    // Läs svaret från servern
    let mut buffer = String::new();
    stream.read_to_string(&mut buffer)?;

    Ok(buffer)
}

#[derive(Clone)]
struct Queue<T: Clone> // En generisk köstruktur som används för att hantera olika typer av order.
where
    T: Clone,
{
    list_of_queue: VecDeque<T>,
}
 

// En trådfunktion som periodiskt läser uppdateringar från en databas och lägger dem i kön för att processas.
fn read_database_thread(orders: Arc<Mutex<Queue<([u16; 4], u16)>>>) {
    loop {
        {
            match server::read_order_updates() {
                Some(order) => orders
                    .lock()
                    .expect("Failed to lock current_orders")
                    .list_of_queue
                    .push_front(order),
                None => {}
            }
        }

        thread::sleep(Duration::from_secs(60));
    }
}

type OrdersFinished = Arc<Mutex<Queue<(Vec<Position>, u16)>>>;
type CurrentOrders = Arc<Mutex<Queue<([u16; 4], u16)>>>;

const MY_IP: &str = "192.168.43.45:7071"; // Servens IP-adress och portnummer.
fn main() {
    let grid = Arc::new(Mutex::new(Grid::new()));
    let mut stream_ = TcpStream::connect("192.168.88.222:12000").unwrap();

    stream_.set_nonblocking(true);
    let stream_ = Arc::new(Mutex::new(stream_));
    let stream_2 = Arc::clone(&stream_);
    let stream_3 = Arc::clone(&stream_);
    let stream_4 = Arc::clone(&stream_);

    let is_order_in_process: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    let current_order: Arc<Mutex<Option<([u16; 4], u16)>>> = Arc::new(Mutex::new(None));
    let finished_orders = OrdersFinished::new(Mutex::new(Queue {
        list_of_queue: VecDeque::new(),
    }));
    let orders_to_process = CurrentOrders::new(Mutex::new(Queue {
        list_of_queue: VecDeque::new(),
    }));
    let sort_request: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    let current_orders_1 = Arc::clone(&orders_to_process);
    let finished_add = finished_orders.clone();
    let grid_clone = Arc::clone(&grid);
    thread::spawn(move || {
        read_database_thread(current_orders_1);
    });

    thread::spawn(move || {
        println!("hh");
        robot::robot_read(stream_2, sort_request, finished_add.clone(), grid);
    });

    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let history_orders = Arc::new(Mutex::new(vec![]));

    let history_orders_1 = Arc::clone(&history_orders);
    let current_order_3 = Arc::clone(&current_order);
    let order_to_process = Arc::clone(&orders_to_process);

    let current_order = Arc::clone(&current_order);
    let history = Arc::clone(&history_orders);
    std::thread::spawn(move || {
        thread::sleep(Duration::from_secs(10));
        let mut grid = Arc::clone(&grid_clone);
        process_order_queue(
            Arc::clone(&grid),
            Arc::clone(&orders_to_process),
            Arc::clone(&is_order_in_process),
            Arc::clone(&current_order),
            stream_4,
        );
        process_finished_order(
            Arc::clone(&finished_orders),
            Arc::clone(&history_orders),
            Arc::clone(&current_order.clone()),
        );
    });
    graphic::run(
        stream_3,
        history_orders_1.clone(),
        current_order_3.clone(),
        order_to_process.clone(),
    )
    .unwrap();
    println!("does it even go out of graphic?");
}

fn process_finished_order(
    finished_orders: OrdersFinished,
    history_orders: Arc<Mutex<Vec<([u16; 4], u16)>>>,
    current_order: Arc<Mutex<Option<([u16; 4], u16)>>>,
) {
    let orders_done_len = finished_orders.lock().unwrap().list_of_queue.len();
    if orders_done_len > 0 {
        let order_done = finished_orders
            .lock()
            .unwrap()
            .list_of_queue
            .pop_front()
            .unwrap();

        let d = current_order.lock().unwrap().clone().unwrap();
        history_orders.lock().unwrap().push((d.0, d.1));
        *current_order.lock().unwrap() = None;
        server::send_order_done_db(order_done.0, order_done.1 as u32);
    }
}

fn process_order_queue(
    grid: Arc<Mutex<Grid>>,
    orders_to_process: CurrentOrders,
    is_order_process: Arc<Mutex<bool>>,
    current_order: Arc<Mutex<Option<([u16; 4], u16)>>>,
    stream: Arc<Mutex<TcpStream>>,
) {
    let is_order_process_c = is_order_process.lock().unwrap().clone();
    if !is_order_process_c {
        let order_queue_len = orders_to_process.lock().unwrap().list_of_queue.len();
        if order_queue_len > 0 {
            let g = grid.lock();
            *is_order_process.lock().unwrap() = true;
            let order_to_send = orders_to_process
                .lock()
                .unwrap()
                .list_of_queue
                .pop_front()
                .unwrap();
            *current_order.lock().unwrap() = Some(order_to_send.clone());
            let positions = g.unwrap().get_positions_for_order(order_to_send.0);
            robot::send_order(order_to_send.1 as u8, positions, stream, );
        }
    }
}
