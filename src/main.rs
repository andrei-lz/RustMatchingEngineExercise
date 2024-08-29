use std::io::{self, BufRead};
use std::cmp::Ordering;

type OrderId = u64;
type Price = u32;
type Qty = u32;

#[derive(PartialEq, Eq)]
enum OrderType {
    Buy,
    Sell,
}

#[derive(Eq)]
struct Order {
    id: OrderId,
    order_type: OrderType,
    price: Price,
    quantity: Qty,
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
fn main()
{
    let mut order_book:Vec<Order> = Vec::new();

    let stdin: io::Stdin = io::stdin(); // Spec mentioned stdin. Normally I would have
    // Multiple threads sending inputs concurrently and write a FIFO sequencer to handle
    // this incoming traffic and pass it through. (as part of the market data consumer)

    for line in stdin.lock().lines() {
        // Rudimentary decoder for the input
        let input = line.unwrap();
        if input.is_empty() { break; } //Just to end the program nicely
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() < 6 { continue; } // Handling for some erroneous input

        let order_type: OrderType = match parts[1] {
            "Buy" => OrderType::Buy,
            "Sell" => OrderType::Sell,
            _ => continue, // Ignore and try again so input is robust
        };

        let mut order = Order {
            id: parts[0].trim_end_matches(':').parse::<OrderId>().unwrap(),
            order_type: order_type,
            price: parts[5].parse::<Price>().unwrap(),
            quantity: parts[2].parse::<Qty>().unwrap(),
        };

        if order.order_type == OrderType::Sell {
            // Insert in the order book for use for later
            let pos = order_book.binary_search(&order).unwrap_or_else(|e| e);
            order_book.insert(pos, order);
        }
        else { // If it's a buy order
            
            while !order_book.is_empty() {

                if order.price < order_book[0].price { break; } // There's no match on the price of the buy order    

                if order_book[0].quantity >= order.quantity { // One sell order is enough to fulfill

                    // Trade data structure not needed for the purposes and intents of this application
                    println!("Trade: {} BTC @ {} USD between {} and {}",
                        order.quantity,
                        order_book[0].price,
                        order.id,
                        order_book[0].id);
                    
                    order_book[0].quantity -= order.quantity;
                    break; // Trade fulfilled    
                }
                else { // Need multiple sell orders to fulfill this trade
                    
                    println!("Trade: {} BTC @ {} USD between {} and {}",
                        order_book[0].quantity,
                        order_book[0].price,
                        order.id,
                        order_book[0].id);

                    order.quantity -= order_book[0].quantity; // This is to keep track of how much is left to fulfill on the next iteration
                    order_book.remove(0);
                    
                }
            }
        }

    }
}