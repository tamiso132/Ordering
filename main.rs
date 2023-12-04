use std::{
    net::TcpStream,
    sync::{Arc, Mutex},
};

// Importera nödvändiga bibliotek för serialisering och deserialiserinf av JSON
use serde::{Deserialize, Serialize};
use serde_derive::{Deserialize, Serialize};

const ORDERS: Vec<u8> = vec![];

struct Orders {
    orders: Arc<Mutex<u8>>,
}

// Definiera en struktur för att representera position (rad och kolumn)
#[derive(Debug, Serialize, Deserialize)]
struct Position {
    row: u32,
    column: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Order {
    id: u32,
    amount: Vec<ProductOrder>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ProductOrder {
    product_type: String,
    total_product_amount: u16,
}

enum SendType {
    Order, // send order info to robot
    Sort,  // send sort info to robot
}

enum Receivetype {
    OrderConfirmaton, // the order is done
    SortConfirmation, // confirm I put into lager
    SortRequest,      // where to place
}

// process, läsa om ny order har kommit,
// proccess, läsa info om robot
// main process,

fn read_order_from_database() {
    // läser från databasen för en order

    // let mut stream = TcpStream::connect("127.0.0.1:34254")?;

    // stream.write(&[1])?;
    // stream.read(&mut [0; 128])?;

    // gör om json format till class
    let example_json = "example";

    let order = serde_json::from_str::<Order>(example_json).unwrap();

    let mut blue = 0; // antal blå i beställning
    let mut red = 0;
    let mut green = 0;
    let mut yellow = 0;

    for e in order.amount {
        let amount = e.total_product_amount; // antal av den färgen
        let product_type = e.product_type; // vilken typ av färg

        match product_type.as_str() {
            "Red block" => red += amount,
            "Yellow block" => yellow += amount,
            "Green block" => green += amount,
            "Blue block" => blue += amount,
            _ => {
                panic!("FICK KONSTIG info från database {}", product_type) // databasen skickar fel info
            }
        }
    }
    // hitta positioner för de olika färgerna
}

fn send_order(positions: Vec<Position>, order_id: String) {}

fn send_robot() {
    // läs från databasen
    // om de finns beställning
    // skicka Order till robot
}

fn interpret_robot() {
    let typ = 1;

    let order_type = match typ {
        0 => {
            // skicka till servern att beställning är klar
            // ta bort den från listan
        }
        1 => {
            // uppdatera listan
            // skicka uppdatering till servern
        }
        2 => {
            // hitta en ledig position,
            // skicka den lediga platsen till robotsystemet
        }
    };
}

// Huvudfunktion som simulerar skickande och mottagande av olika meddelanden
fn main() {
    std::thread::spawn(|| loop {
        read_order_from_database();
    });

    std::thread::spawn(|| loop {
        interpret_robot();
    });

    let x = Mutex::new(50);
    {
        let mut y = x.lock().unwrap();
        *y = 10;
    }

    ORDERS.push(50);

    // // Simulera skickande av olika meddelanden
    // let pickup_order_command = PickupOrderCommand {
    //     command: "pickup_order".to_string(),
    //     order_id: "12345".to_string(),
    //     items: vec![
    //         Ttem {
    //             quantity: 3,
    //             position: Position { row: 2, column: 3 },
    //             position: Position { row: 2, column: 3 },
    //         },
    //         Item {
    //             quantity: 1,
    //             position: Position { row: 4, column: 5 },
    //         },
    //     ],
    // };

    // // Skicka beställningsmeddelandet över nätverket till robotsystemet
    // let pickup_order_json = serde_json::to_string(&pickup_order_command).unwrap();
    // // Skicka pickup_order_json till robotsystemet

    // // Motta svar från robotsystemet och konvertera det till struktur
    // let pickup_order_response: PickupOrderResponse =
    //     serde_json::from_str(&received_response_json).unwrap();
    // // Hantera svar från robotsystemet (t.ex. skriv ut eller behandla ytterligare)

    // let new_item_in_stock_message = NewItemInStockMessage {
    //     command: String::from("new_item_in_stock"),
    //     color: String::from("red"),
    // };

    // // Skicka meddelandet om ny vara i lager över nätverket till PC
    // let new_item_in_stock_json = serde_json::to_string(&new_item_in_stock_message).unwrap();
    // // Skicka new_item_in_stock_json till PC

    // // Motta svar från PC och konvertera det till struktur
    // let placement_request: PlacementRequest = serde_json::from_str(&received_request_json).unwrap();
    // // Hantera förfrågan från PC (t.ex. skriv ut eller behandla ytterligare)

    // // Exempel: Hantera placering och skicka svar tillbaka till robotsystemet
    // let placement_response = handle_placement_request(&placement_request);

    // // Konvertera placeringssvaret till JSON
    // let placement_response_json = serde_json::to_string(&placement_response).unwrap();
    // // Skicka placement_response_json till robotsystemet

    // // Hantera olika meddelanden
    // let pickup_order_response = handle_pickup_order(&pickup_order_command);
    // let placement_request = handle_new_item_in_stock(&new_item_in_stock_message);
    // let placement_response = handle_placement_request(&placement_request);
}
