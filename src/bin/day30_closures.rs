fn main() {
    let tax_rate = 0.10; // 10% tax

    // this is a closure.
    // it is a function defined inside 'main' that can see 'tax_rate'.
    // |income| is the input argument.
    let calculate_tax = |income: f64| -> f64 {
        income * tax_rate
    };

    let my_income = 5000.0;
    let tax_due = calculate_tax(my_income);

    println!("income: ${}, tax due:${}", my_income, tax_due);
}