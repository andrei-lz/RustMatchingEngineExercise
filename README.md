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
