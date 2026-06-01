use std::collections::HashMap;

enum Side {
    Buy,
    Sell,
}

fn build_book(orders: Vec<(u64, u64)>, side: Side) -> (Vec<(u64, u64)>, Option<u64>) {
    let mut levels: HashMap<u64, u64> = HashMap::new();

    for (price, qty) in orders {
        *levels.entry(price).or_insert(0) += qty;
    }

    let mut book: Vec<(u64, u64)> = levels.into_iter().collect();

    match side {
        Side::Buy => {
            book.sort_by(|a, b| b.0.cmp(&a.0));
        }

        Side::Sell => {
            book.sort_by(|a, b| a.0.cmp(&b.0));
        }
    }

    let best_price = book.first().map(|(price, _)| *price);

    (book, best_price)
}

fn book_snapshot(
    bid_levels: &[(u64, u64)],
    ask_levels: &[(u64, u64)],
    depth: usize,
) -> (Vec<(u64, u64)>, Vec<(u64, u64)>, u64) {
    let top_bids: Vec<(u64, u64)> = bid_levels.iter().take(depth).copied().collect();
    let top_asks: Vec<(u64, u64)> = ask_levels.iter().take(depth).copied().collect();

    let bid_qty: u64 = top_bids.iter().map(|(_, q)| q).sum();
    let ask_qty: u64 = top_asks.iter().map(|(_, q)| q).sum();

    let imbalance: u64 = bid_qty * 100 / (bid_qty + ask_qty);

    (top_bids, top_asks, imbalance)
}
