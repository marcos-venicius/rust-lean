pub fn new() {
    // let x = 5; // this broken due immutability
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
