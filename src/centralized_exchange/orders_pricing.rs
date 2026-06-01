enum Status {
    REJECTED,
    IMMEDIATE,
    RESTING,
}

pub struct Order<'a> {
    order_type: String,
    side: &'a str,
    price: u64,
    qty: u64,
    best_bid: u64,
    best_ask: u64,
}

pub fn process_incoming_orders(order: Order) -> &'static str {
    if order.order_type != "MARKET" && order.order_type != "LIMIT" {
        return "REJECTED";
    }
    if order.side != "BUY" && order.side != "SELL" {
        return "REJECTED";
    }
    if order.qty == 0 {
        return "REJECTED";
    }
    if order.order_type == "LIMIT" && order.price == 0 {
        return "REJECTED";
    }

    if order.order_type == "MARKET" {
        return "IMMEDIATE";
    }
    if order.side == "BUY" && order.price >= order.best_ask {
        return "IMMEDIATE";
    }
    if order.side == "SELL" && order.price <= order.best_bid {
        return "IMMEDIATE";
    }

    "RESTING"
}

fn order_economics(
    best_bid: u64,
    best_ask: u64,
    price: u64,
    qty: u64,
    fee_bps: u64,
) -> (u64, u64, u64, u64) {
    let spread = best_ask - best_bid;
    let midprice = (best_bid + best_ask) / 2;
    let notional = price * qty;
    let fee = notional * fee_bps / 10000;

    return (spread, midprice, notional, fee);
}

fn sort_buy_orders(orders: &[(u64, u64)]) -> Vec<usize> {
    let mut indices: Vec<usize> = (0..orders.len()).collect();
    indices.sort_by(|&a, &b| {
        orders[b]
            .0
            .cmp(&orders[a].0)
            .then(orders[a].1.cmp(&orders[b].1))
    });
    indices
}

fn sort_sell_orders(orders: &[(u64, u64)]) -> Vec<usize> {
    let mut indices: Vec<usize> = (0..orders.len()).collect();
    indices.sort_by(|&a, &b| {
        orders[a]
            .0
            .cmp(&orders[b].0)
            .then(orders[a].1.cmp(&orders[b].1))
    });
    indices
}
