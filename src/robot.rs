

enum SendType {
    Order, // send order info to robot
    Sort,  // send sort info to robot
}


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
    send_and_receive_data("dd", person_json);
}

fn send_sort(positions:Position){
    let person_json = json!({
        "command": SendType::Sort,
        "positions": position,
    });
    send_and_receive_data("dd", person_json);
}


fn receive_orderconfrimation(orderconfrimation, order_id: u8){
    let person_json = json!({
      "command": Receivetype::OrderConfirmation, 
      "order_id": order_id,
      "orderconfirmation": orderconfrimation,

    });
    

}

fn receive_sortconfrimation(sortconfirmation){
  let person_json = json!({
    "command": Receivetype::SortConfirmation,
    "sortconformation": sortconformition,
   });  
}

fn received_sortrequest(color:Color){
    let oerson_json = json({
        "command":Receivetype::SortRequest
        "colors": color,
    });
}
   