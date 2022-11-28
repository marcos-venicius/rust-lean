
fn fibonacci(number: u32) {
    let mut n1: u32 = 0;
    let mut n2: u32 = 1;

    while n1 <= number {
        print!("{n1} ");

        let temp_n2 = n2;

        n2 = n1 + n2;
        n1 = temp_n2;
    }

    println!("");
}

fn main() {
    fibonacci(144);
    fibonacci(233);
    fibonacci(1000);
}
