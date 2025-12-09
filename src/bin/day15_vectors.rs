struct LandPlot {
    size: u64,
    price: u64,
}

fn main() {
    let mut portfolio: Vec<LandPlot> = Vec::new();

    portfolio.push(LandPlot { size: 1000, price: 50000});
    portfolio.push(LandPlot { size: 500, price: 25000});
    portfolio.push(LandPlot { size: 200, price: 10000});

    println!("portfolio contains {} plots", portfolio.len());

    let mut total_value = 0;

    for plot in &portfolio {
        // The Logic: Add this plot's price to the running total
        total_value += plot.price; 
    }

    println!("total portfolio value: {} sol", total_value);
}
