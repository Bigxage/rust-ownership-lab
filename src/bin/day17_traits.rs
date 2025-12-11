trait Summary {
    fn summarize(&self) -> String;
}

struct LandPlot {
    size: u64,
    price: u64,
}

impl Summary for LandPlot {
    fn summarize (&self) -> String {
        return format!("{} sqm plot selling for {} sol", self.size, self.price);
    }
}

fn main() {
    let lekki_plot = LandPlot { size: 1000, price: 5000 };

    println!("asset report: {}", lekki_plot.summarize());
}