use rand::Rng;

pub fn new() {
    let number_to_be_compared = rand::thread_rng()
        .gen_range(0..=100);

    let mut attempts = 0;
    let mut random_number = rand::thread_rng()
        .gen_range(0..=100);

    println!("GENERATED NUMBER {}", number_to_be_compared);

    while random_number != number_to_be_compared {
        println!("[{:0>3}] generated: {:0>3}, diff: {:0>3}", attempts + 1, random_number, number_to_be_compared - random_number);

        random_number = rand::thread_rng()
            .gen_range(0..=100);

        attempts += 1;
    }

    println!("GENERATED EQUAL after {} attempts", attempts);
}

