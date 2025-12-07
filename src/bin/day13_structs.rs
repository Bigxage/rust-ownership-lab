struct LandPlot {
    size: u64,
    price: u64,
    is_sold: bool,
}

impl LandPlot {
    fn new(size:u64, price: u64) -> LandPlot{
        return LandPlot{
            size: size,
            price: price,
            is_sold: false,
        };
    }

    fn calculate_fee(&self) -> u64 {
        return self.price * 5 / 100;
    }
}

fn main() {
    let lekki_plot = LandPlot::new(1000,50000);

    println!("Plot size: {} sqm", lekki_plot.size);
    println!("Plot Price: {} SOL", lekki_plot.price);
    println!("Is it sold? {}", lekki_plot.is_sold);

    let fee = lekki_plot.calculate_fee();
    println!("Agency Fee (5%): {} SOL", fee);
}