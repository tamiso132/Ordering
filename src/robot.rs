use serde_derive::{Serialize, Deserialize};
use serde_json::json;

use crate::{send_and_receive_data, server::Position};


#[derive(Serialize, Deserialize)]
enum SendType {
    Order, // send order info to robot
    Sort,  // send sort info to robot
}


#[derive(Serialize, Deserialize)]
enum Receivetype {
    OrderConfirmation, // the order is done
    SortConfirmation, // confirm I put into lager
    SortRequest,      // where to place
}

fn send_order(order_id: u8,positions:Vec<Position>){
    let person_json = json!({
        "command": SendType::Order,
        "order id": order_id,
        "positions": positions
    });
    send_and_receive_data("dd", person_json.to_string().as_str());
}

fn send_sort(positions:Position){
    let person_json = json!({
        "command": SendType::Sort,
        "positions": positions,
    });
    send_and_receive_data("dd", person_json.as_str().unwrap());
}


fn receive_orderconfrimation(order_id: u8){
    let person_json = json!({
      "command": Receivetype::OrderConfirmation, 
      "order_id": order_id,
    });
    

}

fn receive_sortconfrimation(){
  let person_json = json!({
    "command": Receivetype::SortConfirmation,
   });  
}

fn received_sortrequest(){
    let oerson_json = json!({
        "command":Receivetype::SortRequest
    });
}
   