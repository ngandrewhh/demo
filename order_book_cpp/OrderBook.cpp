#include <math.h>

#include <fstream>
#include <iostream>
#include <regex>
#include <string>
#include <unordered_map>
#include <vector>

using namespace std;

// types and helper functions
enum OrderSide
{
    BUY,
    SELL,
};
enum OrderCategory
{
    NEW,
    TRADE,
    CANCEL
};
enum OrderStatus
{
    ACTIVE,
    PARTIAL,
    COMPLETED,
    CANCELLED,
    UNINITIALIZED
};

static unordered_map<string, OrderSide> const mapSide = {
    {"BUY", OrderSide::BUY}, {"SELL", OrderSide::SELL}};
static unordered_map<string, OrderCategory> const mapCategory = {
    {"NEW", OrderCategory::NEW},
    {"TRADE", OrderCategory::TRADE},
    {"CANCEL", OrderCategory::CANCEL}};

// globals
using id_quantity = tuple<string, int>;
regex OrderRE("([A-z]+)|(\\d+\\.?\\d*)");
bool INTERACTIVE_MODE = false;
bool VERBOSE = false;
int BOOK_LIMIT = 5;

class Order
{
    long long epoch;
    string orderId;
    string symbol;
    OrderSide orderSide;
    string orderSideStr;
    OrderCategory orderCategory;
    string orderCategoryStr;
    int price;
    string priceStr;
    int quantity;

    Order(string epoch, string id, string symbol, string side, string category,
          string price, string quantity)
    {
        // Default constructor. Should only be used when loading data with
        // Order::fromString().
        this->epoch = stoll(epoch);
        this->orderId = id;
        this->symbol = symbol;
        this->orderSide = mapSide.at(side);
        this->orderSideStr = side;
        this->orderCategory = mapCategory.at(category);
        this->orderCategoryStr = category;
        this->price = static_cast<int>(stof(price) * 1000);
        this->priceStr = price;
        this->quantity = stoi(quantity);
        // cout << "Order initiated successfully" << endl;
    }

    void modifyPrice(int price)
    {
        this->price = price;
    }

public:
    Order(long long epoch, string id, string symbol, OrderSide side,
          OrderCategory category, int price, int quantity) : epoch{epoch}, orderId{id}, symbol{symbol}, orderSide{side}, orderCategory{category}, price{price}, quantity{quantity}
    {
        // Public facing constructor with proper data types.
        // Used for partial construction from other orders.
    }

    static Order fromString(string s)
    {
        vector<string> v;
        auto captures = sregex_iterator(s.begin(), s.end(), OrderRE);
        auto end = sregex_iterator();

        for (auto c = captures; c != end; ++c)
        {
            string str = (*c).str();
            // cout << str << endl;
            v.push_back(str);
        }

        return Order(v[0], v[1], v[2], v[3], v[4], v[5], v[6]);
    }

    string getID() { return this->orderId; }
    string getUID() { return to_string(this->orderSide) + ";" + this->orderId; }

    friend ostream &operator<<(std::ostream &out, Order const &order)
    {
        out << order.epoch << ';' << order.orderId << ';' << order.orderCategoryStr
            << " " << order.orderSideStr << " $SYM " << order.symbol << ";"
            << order.quantity << " @" << order.priceStr;
        return out;
    }

    friend class OrderBook;
};

// template <typename T>
// vector<T> e(vector<T> v, typename vector<T>::iterator it) {
//     v.erase(it);
//     vector<T> vn = v;
//     return vn;
// }

class OrderBook
{
    string symbol;
    long long lastEpoch;
    long long lastSerialized;
    int lastTradePrice;
    int lastTradeQuantity;
    vector<int> buyPrices;
    vector<int> sellPrices;
    vector<string> snapshots;
    unordered_map<int, vector<id_quantity>> buyQueue;
    unordered_map<int, vector<id_quantity>> sellQueue;
    unordered_map<string, OrderStatus> orderStatus;

    // Add order to buy.
    void addBuyOrder(Order o)
    {
        // locate position of price in queue.
        auto it = this->buyPrices.begin();
        for (; it != this->buyPrices.end(); ++it)
        {
            if (o.price >= *it)
            {
                break;
            }
        }

        if (it == this->buyPrices.end())
        {
            // insert to back of queue.
            this->buyPrices.push_back(o.price);
        }
        else if (*it == o.price)
        {
            // no-op for prices if price exists
        }
        else
        {
            // insert to one place after queue
            this->buyPrices.insert(it, o.price);
        }

        // add to price-queue map
        if (this->buyQueue.find(o.price) == this->buyQueue.end())
        {
            // if not exist, create vector and map price to vec
            vector<id_quantity> vi;
            vi.push_back(make_tuple(o.orderId, o.quantity));
            this->buyQueue[o.price] = vi;
        }
        else
        {
            // add to existing queue
            this->buyQueue[o.price].push_back(make_tuple(o.orderId, o.quantity));
        }

        // update order status
        this->orderStatus[o.getUID()] = OrderStatus::ACTIVE;
    }

    // Add order to sell.
    void addSellOrder(Order o)
    {
        // locate position of price in queue.
        auto it = this->sellPrices.begin();
        for (; it != this->sellPrices.end(); ++it)
        {
            if (o.price <= *it)
            {
                break;
            }
        }

        if (it == this->sellPrices.end())
        {
            // insert to back of queue.
            this->sellPrices.push_back(o.price);
        }
        else if (*it == o.price)
        {
            // no-op for prices if price exists
        }
        else
        {
            // insert to one place after queue
            this->sellPrices.insert(it, o.price);
        }

        // add to price-queue map
        if (this->sellQueue.find(o.price) == this->sellQueue.end())
        {
            // if not exist, create vector and map price to vec
            vector<id_quantity> vi;
            vi.push_back(make_tuple(o.orderId, o.quantity));
            this->sellQueue[o.price] = vi;
        }
        else
        {
            // add to existing queue
            this->sellQueue[o.price].push_back(make_tuple(o.orderId, o.quantity));
        }

        // update order status
        this->orderStatus[o.getUID()] = OrderStatus::ACTIVE;
    }

    // Cancel any ACTIVE or PARTIAL order.
    void cancelOrder(Order o)
    {
        string uid = o.getUID();
        auto *queue =
            o.orderSide == OrderSide::BUY ? &this->buyQueue : &this->sellQueue;
        auto *prices =
            o.orderSide == OrderSide::BUY ? &this->buyPrices : &this->sellPrices;

        // if uid does not exist, returns
        if (this->orderStatus.find(uid) == this->orderStatus.end())
        {
            if (VERBOSE)
            {
                cerr << "OrderBook::cancelOrder() error: uid does not exist" << endl;
            }

            return;
        }

        // if uid status is not ACTIVE, returns
        if ((this->orderStatus[uid] == OrderStatus::COMPLETED) ||
            (this->orderStatus[uid] == OrderStatus::CANCELLED) ||
            (this->orderStatus[uid] == OrderStatus::UNINITIALIZED))

        {
            if (VERBOSE)
            {
                cerr << "OrderBook::cancelOrder() error: order is not active: " << o
                     << endl;
            }

            return;
        }

        // remove the captioned trade, located with price and side and id
        for (auto it = (*queue)[o.price].begin(); it != (*queue)[o.price].end(); ++it)
        {
            if ((get<0>(*it) == o.orderId) && (get<1>(*it) == o.quantity))
            {
                (*queue)[o.price].erase(it);

                // optimization - remove the price level entirely from price list if
                // price queue at certain level becomes 0, this helps actively
                // organise the top 5 prices
                if ((*queue)[o.price].size() == 0)
                {
                    for (auto pit = (*prices).begin(); pit != (*prices).end(); ++pit)
                    {
                        if (*pit == o.price)
                        {
                            (*prices).erase(pit);
                            break;
                        }
                    }
                }
                this->orderStatus[uid] = OrderStatus::CANCELLED;
                break;
            }
        }
    }

    // Trade against book.
    void tradeOrder(Order o)
    {
        /*
           Not so sure about the nature of this order.
           It seems like not an market order.
           Who is the person trading against?
        */
        if (VERBOSE)
        {
            cout << "OrderBook::tradeOrder() processing: " << o << endl;
        }

        string uid = o.getUID();
        auto *queue =
            o.orderSide == OrderSide::BUY ? &this->buyQueue : &this->sellQueue;
        auto *prices =
            o.orderSide == OrderSide::BUY ? &this->buyPrices : &this->sellPrices;

        if (VERBOSE)
        {
            cout << "OrderBook::tradeOrder() [Before]" << endl;
            this->query(BOOK_LIMIT);
        }

        // only acts if order status is ACTIVE or PARTIAL
        if ((this->orderStatus[uid] == OrderStatus::ACTIVE) ||
            (this->orderStatus[uid] == OrderStatus::PARTIAL))
        {
            // find the right entry in the price queue
            auto it = (*queue)[o.price].begin();
            for (; it != (*queue)[o.price].end(); ++it)
            {
                if (get<0>(*it) == o.orderId)
                {
                    break;
                }
            }

            // process the entry
            if (get<1>(*it) == o.quantity)
            {
                this->orderStatus[uid] = OrderStatus::COMPLETED;
                (*queue)[o.price].erase(it);

                // optimization - remove the price level entirely from price list if
                // price queue at certain level becomes 0, this helps actively organise
                // the top 5 prices
                if ((*queue)[o.price].size() == 0)
                {
                    for (auto pit = (*prices).begin(); pit != (*prices).end(); ++pit)
                    {
                        if (*pit == o.price)
                        {
                            (*prices).erase(pit);
                            break;
                        }
                    }
                }
            }
            else if (get<1>(*it) > o.quantity)
            {
                this->orderStatus[uid] = OrderStatus::PARTIAL;
                *it = make_tuple(o.orderId, get<1>(*it) - o.quantity);
            }
            else
            {
                cerr << "OrderBook::tradeOrder() error: order quantity > order on book!"
                     << endl;
            }
        }

        if (VERBOSE)
        {
            cout << "OrderBook::tradeOrder() [After]" << endl;
            this->query(BOOK_LIMIT);
        }
        this->lastTradePrice = o.price;
        this->lastTradeQuantity = o.quantity;
    }

public:
    OrderBook(string symbol) : symbol{symbol} {
        lastEpoch = 0;
        lastSerialized = -1;
        lastTradePrice = -1;
        lastTradeQuantity = -1;
        snapshots.reserve(1024);

        std::ofstream f;
        f.open("snapshots.txt", std::ofstream::out | std::ofstream::trunc);
        f.close();
    };

    // Return top bid and ask in human readable format.
    void peek()
    {
        cout << "TOP BID: "
             << (this->buyPrices.size() > 0 ? to_string(this->buyPrices[0] / 1000.0)
                                            : "n/a")
             << "; "
             << "TOP ASK: "
             << (this->sellPrices.size() > 0
                     ? to_string(this->sellPrices[0] / 1000.0)
                     : "n/a")
             << endl;
    }

    int bestBid()
    {
        return this->buyPrices.size() > 0 ? this->buyPrices[0] : 0;
    }

    int bestAsk()
    {
        return this->sellPrices.size() > 0 ? this->sellPrices[0] : INT_MAX;
    }

    bool askGtBid()
    {
        if ((this->buyPrices.size() > 0) && this->sellPrices.size() > 0)
        {
            return (this->sellPrices[0] > this->buyPrices[0]);
        }
        return true;
    }

    bool bidGtAsk()
    {
        if ((this->buyPrices.size() > 0) && this->sellPrices.size() > 0)
        {
            return (this->buyPrices[0] > this->sellPrices[0]);
        }
        return false;
    }

    // Look up order status.
    OrderStatus lookup(Order o)
    {
        string uid = o.getUID();
        if (this->orderStatus.find(uid) != this->orderStatus.end())
        {
            return this->orderStatus[uid];
        }
        return OrderStatus::UNINITIALIZED;
    }

    // This validates whether the entered price makes sense.
    // If the price does not makes sense, block the trade instead.
    // Such way is easier to accomplish than allowing a logic to modify the price,
    // as this will meddle with the price queue.
    bool validatePrice(Order &o)
    {
        if ((o.orderSide == OrderSide::BUY) && (o.price > this->bestAsk()))
        {
            this->peek();
            cout << "OrderBook::validatePrice() error: price on BUY order greater than best ask price: " << o << endl;
            // o.modifyPrice(this->bestAsk());
            // cout << "Press Enter to Continue..." << endl;
            // cin.ignore();
            return true;
        }
        if ((o.orderSide == OrderSide::SELL) && (o.price < this->bestBid()))
        {
            this->peek();
            cout << "OrderBook::validatePrice() error: price on SELL order less than best bid price: " << o << endl;
            // o.modifyPrice(this->bestBid());
            // cout << "Press Enter to Continue..." << endl;
            // cin.ignore();
            return true;
        }
        return false;
    }

    // Generic entrance to processing an order by OrderCategory & OrderSide.
    void process(Order o)
    {
        // error catching
        if (o.symbol != this->symbol)
        {
            return;
        }
        if (o.quantity <= 0)
        {
            return;
        }

        // ensuring orderId cannot be reused, given the uniqueness.
        // here we take the path of blocking any order that uses same id,
        // even if the previous order is cancelled.
        OrderStatus status = this->lookup(o);
        if (o.orderCategory == OrderCategory::NEW)
        {
            if (!(status == OrderStatus::UNINITIALIZED))
            {
                if (VERBOSE)
                {
                    cerr << "OrderBook::process() error: orderCategory NEW but orderStatus is not UNINITIALIZED: " << o << endl;
                }
                return;
            }
        }

        if (o.orderCategory == OrderCategory::TRADE)
        {
            if (!((status == OrderStatus::ACTIVE) || (status == OrderStatus::PARTIAL)))
            {
                if (VERBOSE)
                {
                    cerr << "OrderBook::process() error: orderCategory TRADE but orderStatus is not (ACTIVE|PARTIAL): " << o << endl;
                }
                return;
            }
        }

        if (o.orderCategory == OrderCategory::NEW)
        {
            if (o.orderSide == OrderSide::BUY)
            {
                this->addBuyOrder(o);
            }
            if (o.orderSide == OrderSide::SELL)
            {
                this->addSellOrder(o);
            }
        }
        else if (o.orderCategory == OrderCategory::CANCEL)
        {
            this->cancelOrder(o);
        }
        else if (o.orderCategory == OrderCategory::TRADE)
        {
            this->tradeOrder(o);
            if (INTERACTIVE_MODE)
            {
                cout << "Press Enter to Continue..." << endl;
                cin.ignore();
            }
        }
        this->lastEpoch = o.epoch;
        // cout << "Processed " << o << endl;
    }

    /*
        This is triggered when the ask > bid condition is hit.
        If the buy order inserted is what triggered the condition, then
        we first match it against the sell, or vice versa.
    */
    void matchAgainst(Order o)
    {
        cout << "OrderBook::matchAgainst(): " << o << endl;
        string uid = o.getUID();
        auto *fromQueue = o.orderSide == OrderSide::BUY ? &this->sellQueue : &this->buyQueue;
        auto *toQueue = o.orderSide == OrderSide::BUY ? &this->buyQueue : &this->sellQueue;
        auto toSide = o.orderSide == OrderSide::BUY ? OrderSide::SELL : OrderSide::BUY;
        // auto toUID = o.orderSide == OrderSide::BUY ? 5 : 4;

        int quantity_acc = 0;
        vector<id_quantity> v;
        for (auto it = (*fromQueue)[o.price].begin();
             it != (*fromQueue)[o.price].end(); ++it)
        {
            quantity_acc += get<1>(*it);
            v.push_back(*it);
            if (quantity_acc >= o.quantity)
            {
                break;
            }
        }

        this->query(BOOK_LIMIT);

        // case: if all prices in the queue is not adequate / just adequate to resolve order
        if (quantity_acc <= o.quantity)
        {
            // creating synthetic trade orders
            // the same way we only need to partially / completely fill,
            // the tradeOrder method will take care.
            // cout << "quantity_acc <= o.quantity branch" << endl;
            Order new_o = Order(o.epoch, o.orderId, o.symbol, o.orderSide, OrderCategory::TRADE,
                                o.price, quantity_acc);
            new_o.orderCategoryStr = "SYNTHETIC_TRADE";
            new_o.orderSideStr = o.orderSideStr;
            this->process(new_o);

            // the other way is completely consumed
            for (auto it = v.begin(); it != v.end(); ++it)
            {
                Order new_o = Order(o.epoch, get<0>(*it), o.symbol, toSide, OrderCategory::TRADE,
                                    o.price, get<1>(*it));
                this->process(new_o);
            }
        }
        else
        // if more than adequate
        {
            // cout << "else branch" << endl;
            // creating synthetic trade orders
            Order new_o = Order(o.epoch, o.orderId, o.symbol, o.orderSide, OrderCategory::TRADE,
                                o.price, o.quantity);
            new_o.orderCategoryStr = "SYNTHETIC_TRADE";
            new_o.orderSideStr = o.orderSideStr;
            this->process(new_o);

            // the other way is consumed, except for the last accumulated
            // order, we only fill partially with ternary into q
            for (auto it = v.rbegin(); it != v.rend(); ++it)
            {
                int q = (it == v.rbegin()) ? get<1>(*it) - (quantity_acc - o.quantity) : get<1>(*it);
                Order new_o = Order(o.epoch, get<0>(*it), o.symbol, toSide, OrderCategory::TRADE,
                                    o.price, q);
                this->process(new_o);
            }
        }
    }

    // returns b || s, works on 0..=limit as input
    string queryLevel(int level)
    {

        string s_b = "";
        if (this->buyPrices.size() > level)
        {
            for (auto bit = this->buyQueue[this->buyPrices[level]].rbegin();
                 bit != this->buyQueue[this->buyPrices[level]].rend(); ++bit)
            {
                s_b += to_string(get<1>(*bit));
                s_b += "; ";
            }
            s_b += " @ ";
            s_b += to_string(this->buyPrices[level] / 1000.0);
        }
        if (s_b.length() < 40)
        {
            s_b.insert(s_b.begin(), 40 - s_b.length(), ' ');
        }

        string s_s = "";
        if (this->sellPrices.size() > level)
        {
            s_s += to_string(this->sellPrices[level] / 1000.0);
            s_s += " @ ";

            for (auto sit = this->sellQueue[this->sellPrices[level]].begin();
                 sit != this->sellQueue[this->sellPrices[level]].end(); ++sit)
            {
                s_s += to_string(get<1>(*sit));
                s_s += "; ";
            }
            // s_s.insert(s_s.end(), 40 - s_s.length(), ' ');
        }

        return s_b + " || " + s_s;
    }

    // prints the buy / sell book side by side.
    void query(int limit = 5)
    {

        string header = "Top ";
        header += to_string(limit);
        header += " Buy";
        header.insert(header.begin(), 40 - header.length(), ' ');
        header += " || Top ";
        header += to_string(limit);
        header += " Sell";

        cout << header << endl;

        for (int i = 0; i < limit; ++i)
        {
            cout << this->queryLevel(i) << endl;
        }
    }

    // prints the buy book and sell book separately.
    void queryTwoPage(int limit = 5)
    {

        cout << "\nTop " << limit << " Buy Level" << endl;

        for (auto it = this->buyPrices.begin(); it != this->buyPrices.end(); ++it)
        {
            if (distance(this->buyPrices.begin(), it) == limit)
            {
                break;
            }
            cout << (*it / 1000.0) << ": ";
            auto entry = this->buyQueue[*it];
            for (id_quantity iq : entry)
            {
                cout << get<1>(iq) << "; ";
            }
            cout << endl;
        }

        cout << "\nTop " << limit << " Sell Level" << endl;
        for (auto it = this->sellPrices.begin(); it != this->sellPrices.end();
             ++it)
        {
            if (distance(this->sellPrices.begin(), it) == limit)
            {
                break;
            }
            cout << (*it / 1000.0) << ": ";
            auto entry = this->sellQueue[*it];
            for (id_quantity iq : entry)
            {
                cout << get<1>(iq) << "; ";
            }
            cout << endl;
        }
    }

    bool serializeTimer() {
        return (this->lastEpoch - this->lastSerialized) > 1000000000;
    }

    // serialize to disk in below format:
    // 1. sym;lastEpoch;lastPrice;lastQty X 
    // 2. (top 5 bids) qty@price X
    // 3. (top 5 asks) qty@price ;
    void serialize()
    {
        this->lastSerialized = this->lastEpoch;
        string s = "";
        s += this->symbol + ";";
        s += to_string(this->lastEpoch) + ";";
        s += to_string(this->lastTradePrice) + ";";
        s += to_string(this->lastTradeQuantity) + " X ";

        // bid queue
        vector<string> bids;
        int i = 0;

        for (auto pit = this->buyPrices.begin(); pit != this->buyPrices.end(); ++pit, ++i)
        {
            if (i == 5)
            {
                break;
            }
            int q = 0;
            for (auto qit = this->buyQueue[*pit].begin(); qit != this->buyQueue[*pit].end(); ++qit)
            {
                q += get<1>(*qit);
            }
            string tmp = (to_string(q) + "@" + to_string(*pit));
            bids.push_back(tmp);
        }

        for (; i != 5; ++i)
        {
            s += "#;";
        }

        for (auto it = bids.rbegin(); it != bids.rend(); ++it)
        {
            s += *it;
            if (it + 1 == bids.rend())
            {
                s += " X ";
            }
            else
            {
                s += ";";
            }
        }

        // ask queue
        vector<string> asks;
        i = 0;

        for (auto pit = this->sellPrices.begin(); pit != this->sellPrices.end(); ++pit, ++i)
        {
            if (i == 5)
            {
                break;
            }
            int q = 0;
            for (auto qit = this->sellQueue[*pit].begin(); qit != this->sellQueue[*pit].end(); ++qit)
            {
                q += get<1>(*qit);
            }
            string tmp = (to_string(q) + "@" + to_string(*pit));
            asks.push_back(tmp);
        }

        for (auto it = asks.begin(); it != asks.end(); ++it)
        {
            s += *it + ";";
        }

        for (; i != 5; ++i)
        {
            s += "#";
            if (!(i + 1 == 5)) {
                s += ";";
            }
        }

        this->snapshots.push_back(s);

        if (this->snapshots.size() >= 1000) {
            this->flush();
        }
    }

    void flush() {
        ofstream f ("snapshots.txt", ios_base::app);
        if (f.is_open()) {
            for (auto it = this->snapshots.begin(); it != this->snapshots.end(); ++it) {
                f << *it << endl;
            }
            f.close();
        }
        this->snapshots.clear();            
    }
};

int main(int argc, char **argv)
{
    INTERACTIVE_MODE = false;
    VERBOSE = false;
    BOOK_LIMIT = 5;

    ifstream in;
    string secName;
    if (argc == 1)
    {
        cout << "Press Enter to Continue..." << endl;
        cin.ignore();
        return -1;
    }
    else
    {
        string s(argv[1]);
        in = ifstream(s + ".log");
        secName = s;
    }

    vector<Order> orders;
    string str;
    int i = 0;

    // Read data
    while (getline(in, str))
    {
        if (str.length() == 0)
        {
            continue;
        }

        // file reader - try/catch is required due to bad input UNK trade direction.
        // these orders are omitted.
        try
        {
            Order order = Order::fromString(str);
            // if (order.get_category() == OrderCategory::TRADE) { break; }
            orders.push_back(order);
            i += 1;
        }
        catch (const exception &e)
        {
            cerr << "main(): line omitted: error <" << e.what() << "> at " << str << endl;
        }
    }

    // Create order book
    i = 0;
    int serialized = 0;
    OrderBook ob = OrderBook(secName);
    for (Order o : orders)
    {
        // cout << "main(): processing: " << o << endl;

        // Pre check
        if (ob.validatePrice(o))
        {
            if (VERBOSE)
            {
                cerr << "\tOrderBook::validatePrice(): NEW order price not meeting best levels: " << o << endl;
            }
            continue;
        }

        ob.process(o);
        i += 1;

        // Periodic query 
        if ((i % 1000) == 999)
        {
            cout << "main(): " << i + 1 << " lines inserted into OrderBook." << endl;
            ob.query();
        }

        // Periodic [second level] serialize to disk
        if (ob.serializeTimer()) {
            serialized += 1;
            ob.serialize();
        }

        // Post check
        if (!ob.askGtBid())
        {
            // For entries at top level of book, we still treat as valid and perform matching.
            if (VERBOSE)
            {
                cerr << "main(): Problematic order: " << o << endl;
            }
            if (ob.bidGtAsk())
            {
                // Should not trigger.
                break;
            }
            ob.matchAgainst(o);
            // cout << "Press Enter to Continue..." << endl;
            // cin.ignore();
        }

        // Should not trigger.
        if (!ob.askGtBid())
        {
            break;
        }
    }

    cout << "main(): " << i << " lines processed from data." << endl;
    cout << "main(): " << i << " lines processed by OrderBook (including errors)." << endl;
    cout << "main(): OrderBook serialized to disk " << serialized << " times." << endl;
    ob.flush();
    ob.query();
}