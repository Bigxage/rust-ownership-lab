struct LandPlot {
    size: u64,
    price: u64,
    is_sold: bool,
}

fn main() {
    let lekki_plot = LandPlot {
        size: 500,
        price: 1000,
        is_sold: false,
    };

    println!("Plot size: {} sqm", lekki_plot.size);
    println!("Price: {} SOL", lekki_plot.price);
    println!("Is it sold? {}", lekki_plot.is_sold);
}