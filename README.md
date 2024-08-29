
I wasn't sure whether I had to count my own unique id for orders coming in or not, I have chosen to take the input literally (hence used the 1, 2, 3... at the start as the id)

Inside, you will find 2 different rust scripts

The first one ```main.rs``` is the actual submission as outlined in the specification (a more simple version)

Only thing to note here is that the output comes out for every line of input. I could have pushed trades onto a vector and printed them all at the end, however, the wording chosen in the specification ```we emit a trade message and``` made me think that I shouldn't bother and I should just emit the trade message whenever it is carried out. I assume this is the placeholder for when you would emit messages between microservices in the bigger project.

The second one named ```two-way-orders.rs``` is a (rudimentary) matching engine that:

(Press Ctrl + Z to signal ending the input on the console and get the trades to print, I'm sure you know this already, just making triple sure :))

1. Continuously matches orders as they come in
2. Keeps track of *both* buy orders and sell orders 
3. This means that if a buy order cannot presently be fulfilled, it is not discarded and instead waits for a sell order to fulfill it

The difference in handling can be seen here:
```
PS D:\Projects\Rust\RustTest> rustc .\src\main.rs
PS D:\Projects\Rust\RustTest> .\main.exe
1: Buy 76 BTC @ 6000 USD
2: Buy 76 BTC @ 6000 USD
3: Buy 76 BTC @ 6000 USD
4: Sell 3000 BTC @ 5000 USD

PS D:\Projects\Rust\RustTest> rustc .\src\two-way-orders.rs
PS D:\Projects\Rust\RustTest> .\two-way-orders.exe
1: Buy 76 BTC @ 6000 USD
2: Buy 76 BTC @ 6000 USD
3: Buy 76 BTC @ 6000 USD
4: Sell 3000 BTC @ 5000 USD

Trade: 76 BTC @ 5000 USD between 1 and 4
Trade: 76 BTC @ 5000 USD between 2 and 4
Trade: 76 BTC @ 5000 USD between 3 and 4
```

This is essentially closer to what you would see in production AFAIK: a basic implementation of a price ladder

Only thing to note here is I wasn't sure which one would end up being faster (not sure how sort is implemented on the backend in rust): 

1. Inserting into a vector using binary search or
```rust
let pos = sell_orders.binary_search(&order).unwrap_or_else(|e| e);
sell_orders.insert(pos, order);
```

2. Pushing and sorting thereafter
```rust
sell_orders.push(order);
sell_orders.sort();
```

At a glance it seems the first method would be faster but both should work.

I was initially storing and matching tickers and currency, however I have decided that is OOS and have assumed only trades on the BTC/USD market are made.

Also a thing to note, I was somewhat restricted by the input being stdin(). If given more time to expand on this task, I would typically spawn multiple threads and have those pump out orders concurrently. I would then write a FIFO sequencer (In the form of a lock-free queue) to catch these in order. However, I think this is far beyond the scope of the exercise so I will stop here.