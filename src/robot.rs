

enum SendType {
    Order, // send order info to robot
    Sort,  // send sort info to robot
}

enum Receivetype {
    OrderConfirmaton, // the order is done
    SortConfirmation, // confirm I put into lager
    SortRequest,      // where to place
}
