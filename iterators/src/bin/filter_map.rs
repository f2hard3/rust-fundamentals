fn main() {
    let stocks = ["nvda", "", "aapl", "", "msft", "goog"];
    let capitalized_stocks: Vec<String> = stocks
        .iter()
        .filter_map(|stock| {
            if stock.is_empty() {
                None
            } else {
                Some(stock.to_uppercase())
            }
        })
        .collect::<Vec<String>>();

    println!("capitalized_stocks: {capitalized_stocks:?}");
}
