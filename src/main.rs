use std::{
    collections::VecDeque,
    io::{Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use server::{read_order_updates, Grid, Position, PositionWithColor};

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
struct Queue<T: Clone>
where
    T: Clone,
{
    list_of_queue: VecDeque<T>,
}

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

const MYIP: &str = "";
fn main() {
    let grid = Arc::new(Mutex::new(Grid::new()));
    let stream_bind = TcpListener::bind(MYIP).unwrap();
    let mut stream = stream_bind.accept().unwrap().0;
    stream.set_nonblocking(true);

    let is_order_in_process: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    let current_order: Arc<Mutex<Option<([u16; 4], u16)>>> = Arc::new(Mutex::new(None));

    let finished_orders = OrdersFinished::new(Mutex::new(Queue {
        list_of_queue: VecDeque::new(),
    }));

    let orders_to_process = CurrentOrders::new(Mutex::new(Queue {
        list_of_queue: VecDeque::new(),
    }));

    let sort_request: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));

    let robot_stream = Arc::new(Mutex::new(stream));

    let current_orders_1 = orders_to_process.clone();

    let finished_add = finished_orders.clone();
    let grid_clone = Arc::clone(&grid);
    thread::spawn(move || {
        read_database_thread(current_orders_1);
    });

    thread::spawn(move || {
        robot::robot_read(
            robot_stream.clone(),
            sort_request,
            finished_add.clone(),
            grid,
        );
    });

    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let history_orders = Arc::new(Mutex::new(vec![]));
    loop {
        thread::sleep(Duration::from_secs(10));
        let grid = Arc::clone(&grid_clone);
        process_order_queue(
            grid.clone(),
            orders_to_process.clone(),
            is_order_in_process.clone(),
            current_order.clone(),
        );
        process_finished_order(
            finished_orders.clone(),
            history_orders.clone(),
            current_order.clone(),
        );
        graphic::run(
            history_orders.clone(),
            current_order.clone(),
            orders_to_process.clone(),
        )
        .unwrap();
    }
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
            robot::send_order(order_to_send.1 as u8, positions);
        }
    }
}
