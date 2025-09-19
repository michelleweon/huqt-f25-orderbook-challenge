# Orderbook Challenge
In this deliverable, your goal is to create an efficient orderbook.

## Background

The most generic exchange is a central limit order book (CLOB) with price time priority. The most basic kind of order is a limit order, where you can express a limit price, a size, and a side. The limit price defines the worst price someone is willing to trade at and the size expresses how many contracts they want to trade. The side, which is bid or ask, expresses whether you want to buy (bid) or sell (ask). For example, a limit order with limit price 20, size 10, and on the bid side expresses that you want to buy 10 contracts with price at most 20.

An orderbook matches orders against other orders. As orders come in, they get matched against orders that have been placed earlier but have not been matched yet. The orders that have not been matched yet are called resting orders. When a new order comes in, it gets repeatedly matched with the earliest placed resting order on the opposite side with the best price until no orders can be matched with it (based on price constraints) or the new order has been fully filled. 

A resting order can also be canceled, meaning it is removed from the book. It can no longer match against new orders.

In an orderbook, you trade two assets between each other. These are the base asset and the quote asset. The price is expressed in the quote asset and the size is expressed in the base asset. Volume is the amount of the base asset that gets traded around, while notional volume is the amount of quote asset that gets traded around.

## Specs

In src/types.rs, you will find this OrderBook trait. It defines the interface that you want to implement.
* new_book is a constructor
* add_order sends a new order with an id into the book, matches the order, removes filled orders, and adds the new order to the book if it is only partially filled.
* cancel_order cancels the order with the specified id. Does nothing if the id is not in the book.
* get_vol returns the total volume traded since creation
* get_ntnl_vol returns the total notional volume traded since creation

The last two functions are there to easily check if your orderbook is working properly. There are test cases in main.rs.

```rust
pub trait OrderBook {
    // create new book
    fn new_book() -> Self;
    // insets order into the orderbook
    fn add_order(&mut self, id: u64, side: Side, size: i64, price: i64);
    // cancel resting order. Does nothing if order doesn't exist
    fn cancel_order(&mut self, id: u64);
    // total traded volume
    fn get_vol(&self) -> i64;
    // total traded notional volume
    fn get_ntnl_vol(&self) -> i64;
}
```

Run the command 
```bash
cargo t --release 
```
to run all tests. If the tests take less than 5 seconds to run, you pass!

To actually implement the orderbook, make a folder src/sample, and implement the struct OrderBookImpl there with the trait OrderBook in src/sample/mod.rs.