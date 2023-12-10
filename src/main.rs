use std::{
    collections::VecDeque,
    io::{Read, Write},
    net::{SocketAddr, TcpStream},
    sync::{Arc, Mutex},
    thread, time::Duration,
};

use server::{Grid, read_order_updates, Position};

// Importera nödvändiga bibliotek för serialisering och deserialiserinf av JSON

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
struct Queue<T: Clone> where T: Clone{
    list_of_queue:Vec<T>,
}

fn read_database_thread(read_order_flag: Arc<Mutex<bool>>, orders:Arc<Mutex<Queue<([u16;4], u16)>>>){
    loop {
        {
            let mut read_flag = read_order_flag.lock().expect("Failed to lock read_order_flag");

            if *read_flag {
                match server::read_order_updates() {
                    Some(order) => {
                        *read_flag = false;
                        orders.lock().expect("Failed to lock current_orders").list_of_queue.push(order);
                    }
                    None => {}
                }
            }
        }

        thread::sleep(Duration::from_secs(60));
    }
}

fn read_robot_thread(){
}

type OrdersFinished = Arc<Mutex<Queue::<(Vec<Position>, u16)>>>;
type CurrentOrders = Arc<Mutex<Queue::<([u16; 4], u16)>>>;

fn main() {
    let grid = Arc::new(Mutex::new(Grid::new()));

    let current_orders = CurrentOrders::new(Mutex::new(Queue { list_of_queue: vec![] }));
    let finished_orders = OrdersFinished::new(Mutex::new(Queue { list_of_queue: vec![] }));

    let read_order_flag = Arc::new(Mutex::new(true));


    let current_orders_1 = current_orders.clone();
    let read_order_flag_1 = read_order_flag.clone();

    let finished_add = finished_orders.clone();

    thread::spawn(move || {
       read_database_thread(read_order_flag_1, current_orders_1);
    });

    thread::spawn(move || {
        robot::robot_read(read_order_flag.clone(), finished_add.clone());
    });

    loop {
        thread::sleep(Duration::from_secs(10));
        process_order_queue(grid.clone(), current_orders.clone());
        process_finished_order(finished_orders.clone());
      
    }
   // server::send_order_done_db(vec![Position{position_x: 3, position_y: 3}], 9);
}

fn process_finished_order(finished_orders: OrdersFinished){
    let mut orders_done = finished_orders.lock().unwrap();
    if orders_done.list_of_queue.len() > 0{
        let order_done = orders_done.list_of_queue.pop().unwrap();
        server::send_order_done_db(order_done.0, order_done.1 as u32);
    }
}

fn process_order_queue(grid:Arc<Mutex<Grid>>,current_orders:CurrentOrders){
    let mut current_orders = current_orders.lock().expect("Failed to lock current_orders");
    let order_queue = &mut current_orders.list_of_queue;

    if order_queue.len() > 0 {
        let g = grid.lock();

       
        let positions = g.unwrap().get_positions_for_order( order_queue[0].0);
        robot::send_order(order_queue[0].1 as u8, positions);
    }
}
