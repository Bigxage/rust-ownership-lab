// 1. Single Generic (Homogeneous)
// Both x and y MUST be the same type 'T'.
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// 2. Double Generic (Heterogeneous)
// x is type 'T', y is type 'U'. They can be different!
#[derive(Debug)]
struct MixedPoint<T, U> {
    x: T,
    y: U,
}

fn main() {
    // Scenario A: Integers (T becomes i32)
    let int_point = Point { x: 5, y: 10 };
    
    // Scenario B: Floats (T becomes f64)
    let float_point = Point { x: 1.5, y: 4.5 };
    
    // ERROR CHECK: Uncommenting the line below causes an error.
    // let fail_point = Point { x: 5, y: 4.5 }; 
    // Why? Because Point<T> enforces x and y to be the SAME type.
    
    // Scenario C: Mixed Types (T is i32, U is f64)
    // This allows mixed precision.
    let mixed = MixedPoint { x: 5, y: 4.5 };

    println!("Integer Point: {:?}", int_point);
    println!("Float Point: {:?}", float_point);
    println!("Mixed Point: {:?}", mixed);
}