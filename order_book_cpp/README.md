## Entry Point
- To run, execute the compiled binary directly with an option `NAME`. For the provided examples, supported names are `TESTA` and `TESTB`.
- To run for data >1mil lines, compiling with -O2 flags might be more efficient.

## Input Data
- Mnemonic: `[field]` `{enumerated_field}`
- Format: `[epoch_nanoseconds]` `[id]` `[symbol]` `{BUY|SELL}` `{NEW|CANCEL|TRADE}` `[price]` `[quantity]`
- Example dataset: `TESTA.log`, `TESTB.log`

## Main Loop
The general loop of the program is read all orders from file into `Order`s, then run the data through an `OrderBook` instance, per `Order` using `validatePrice` then `process`. 

## Program Layout
- There are 2 major classes and 3 enums:
    - class `Order`
        - Basically an helper class to read string based order data from XXX.log into a structured format, allows pretty-printing to cout.
        - Converts `epoch` to `long long`.
        - Converts `side` to `enum` `OrderSide`.
        - Converts `category` to `enum` `OrderCategory`.
        - Converts `price` to an integer representation by * 1000, to prevent floating point operations.
        - Converts `quantity` to integer.
        - Has an helper method `getUID` to obtain a unique id composed of (side + orderId).

    - enum `OrderSide`
        - Takes `BUY` and `SELL` two values.
        - Helps assert to drop `UNK` direction trades in the raw file.
    - enum `OrderCategory`
        - Takes `NEW`, `TRADE`, `CANCEL` three values.
    - enum `OrderStatus`
        - Takes `ACTIVE`, `PARTIAL`, `COMPLETED`, `CANCELLED`, `UNINITIALIZED` 5 values.
        - `ACTIVE` and `PARTIAL` is reserved for active orders. `PARTIAL` is achieved by a `TRADE` order that is does not consume the entire quantity. 
        - `COMPLETED` is reserved for completed orders through `TRADE` order.
        - `CANCELLED` is reserved for cancelled orders through `CANCEL` order. The system does not allow cancelling in a partial manner.
        - `UNINITIALIZED` is reserved for `NEW` orders those unique ID (uid) is not populated in the system yet.
    
    - class `OrderBook`
        - The major class that interacts with orders. 
         
        - variables:
            - `long long` `lastEpoch`
            - `long long` `lastSerialized`
            - `int` `lastTradePrice`
            - `int` `lastTradeQuantity`
            - `vector<int>` `buyPrices`: a vector of bid prices, in descending order (highest ranks first) to improve querying access.
            - `vector<int>` `sellPrices`: a vector of ask prices, in ascending order (lowest ranks first).
            - `vector<string>` `snapshots`: a vector of snapshots, used for bulk flushing of snapshots to disk;
            - `unordered_map<int, vector<id_quantity>>` `buyQueue`: a map to a vector of (`string` `id`, `int` `quantity`) of a specific price on buy orders;
            - `unordered_map<int, vector<id_quantity>>` `sellQueue`: mirror of `buyQueue` but for sell orders;
            - `unordered_map<string, OrderStatus>` `orderStatus`: a map of uid (side + orderId) to `OrderStatus`;
        
        - 4 different order processing methods, with one unified entry point `process` method:
            - `process`: handles some pre-check such as correct `OrderStatus` and `OrderCategory` pairing, before routing to below methods.
            - `addBuyOrder`: locates the price, upsert the price in the `buyPrices`, push the order to the correct vector in `buyQueue`
            - `addSellOrder`: mirror of `addBuyOrder` 
            - `cancelOrder`: cancels the order by looking in the correct `Queue`. if after cancelling the order, there is no more order at the side at given price, also removes the price from the relevant `Prices`.
            - `tradeOrder`: trades the order by looking in the correct `Queue`. If the trade does not consume the entire order quantity, assigns a `PARTIAL` status to order. If after trading the order, there is no more order at the side at given price, also removes the price from the relevant `Prices`.
        
        - 1 order validation method:
            - `validatePrice`: validate the new order added with not violate `ask` > `bid`. A design choice is enacted such that we allow the new order to create a situation such that `ask` == `bid`.
        
        - 1 "synthetic" order processing method:
            - `matchAgainst`: if the condition `ask` == `bid` is triggered, acts as a wrapper to create synthetic orders to resolve the situation, until the condition `ask` > `bid` holds again.

        - helper methods:
            - `lookup`: returns the status of the given UID. returns `UNINITIALIZED` if order is not found in `orderStatus`.
            - `query`: returns a tabulated top N bid ask prices. a wrapper to the `queryLevel` method, which queries the n-th level bid ask prices.
            - `serialize`: returns a snapshot to cache `snapshots`.
            - `flush`: flush `snapshots` to disk when size is at capacity defined.