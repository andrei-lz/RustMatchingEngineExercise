use std::cmp::Ordering;
use std::fmt;
use std::io::{self, BufRead};

type OrderId = u64;
type Price = u32;
type Qty = u32;

#[derive(PartialEq, Eq)]
enum OrderType {
    Buy,
    Sell,
}

impl fmt::Debug for OrderType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match self {
            OrderType::Buy => write!(f, "Buy"),
            OrderType::Sell => write!(f, "Sell")
       }
    }
}

impl fmt::Display for OrderType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match self {
            OrderType::Buy => write!(f, "Buy"),
            OrderType::Sell => write!(f, "Sell")
       }
    }
}

#[derive(Eq)]
struct Order {
    id: OrderId,
    order_type: OrderType,
    price: Price,
    quantity: Qty,
}

impl fmt::Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {} {} BTC @ {} USD", self.id, self.order_type, self.quantity,self.price)
    }
}

impl fmt::Debug for Order {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Order {{ id: {}, order_type: {:?}, price: {}, quantity: {}}}",
            self.id, self.order_type, self.price, self.quantity
        )
    }
}

impl Ord for Order {
    fn cmp(&self, other: &Self) -> Ordering {
        // Implemented order so that we can use cmp for binary search later
        self.price.cmp(&other.price)
    }
}

impl PartialOrd for Order {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Order {
    fn eq(&self, other: &Self) -> bool {
        self.price == other.price
    }
}

struct Trade {
    buy_id: OrderId,
    sell_id: OrderId,
    price: Price, // this should be the sell price.
    quantity: Qty
}
 
impl fmt::Display for Trade {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Trade: {} BTC @ {} USD between {} and {}", self.quantity, self.price, self.buy_id, self.sell_id)
    }
}

impl fmt::Debug for Trade {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Trade: {} BTC @ {} USD between {} and {}", self.quantity, self.price, self.buy_id, self.sell_id)
    }
}

fn main()
{
    let mut trades:Vec<Trade> = Vec::new();
    let mut sell_orders:Vec<Order> = Vec::new();
    let mut buy_orders:Vec<Order> = Vec::new();

    let stdin: io::Stdin = io::stdin(); // Spec mentioned stdin. Normally I would have
    // Multiple threads sending inputs concurrently and write a FIFO sequencer to handle
    // this incoming traffic and pass it through. (as part of the market data consumer)

    for line in stdin.lock().lines() {
        // Decoder for the input
        let input = line.unwrap();
        if input.is_empty() { break; } //Just to end the program nicely
        let parts: Vec<&str> = input.split_whitespace().collect();
        
        let order_type: OrderType = match parts[1] {
            "Buy" => OrderType::Buy,
            "Sell" => OrderType::Sell,
            _ => panic!("Unknown Order Type"),
        };

        let order = Order {
            id: parts[0].trim_end_matches(':').parse::<OrderId>().unwrap(),
            order_type: order_type,
            price: parts[5].parse::<Price>().unwrap(),
            quantity: parts[2].parse::<Qty>().unwrap(),
        };

        // Decoder End

        match order.order_type {
            OrderType::Buy => {
                // Ensure elements are entered in order of cheapest price
                // let pos = buy_orders.binary_search(&order).unwrap_or_else(|e| e);
                // buy_orders.insert(pos, order);
                
                buy_orders.push(order);
                buy_orders.sort();
            }
            OrderType::Sell => {
                // let pos = sell_orders.binary_search(&order).unwrap_or_else(|e| e);
                // sell_orders.insert(pos, order);

                sell_orders.push(order);
                sell_orders.sort();
            }
        }
        
        if sell_orders.is_empty() || buy_orders.is_empty() { continue; } // No matches can be considered if either is empty
        
        while !sell_orders.is_empty() && !buy_orders.is_empty() {

            if sell_orders[0].price > buy_orders[0].price { continue; } // Buyers and sellers aren't matching

            if sell_orders[0].quantity >= buy_orders[0].quantity { // One sell order is enough to fulfill

                trades.push(Trade {
                    buy_id: buy_orders[0].id,
                    sell_id: sell_orders[0].id,
                    price: sell_orders[0].price,
                    quantity: buy_orders[0].quantity,
                });

                sell_orders[0].quantity -= buy_orders[0].quantity;
                buy_orders.remove(0);

            }
            else { // Needs to be fulfilled with multiple sell orders as buy order is too big meaning we remove sell orders
                // and let it iterate again
            
                trades.push(Trade {
                    buy_id: buy_orders[0].id,
                    sell_id: sell_orders[0].id,
                    price: sell_orders[0].price,
                    quantity: sell_orders[0].quantity,
                });

                buy_orders[0].quantity -= sell_orders[0].quantity;
                sell_orders.remove(0);
            
            }
            
        }
        
    }
    
    for trade in trades {
        println!("{}", trade); // Can also use ("{:?}") for debug
    }
}