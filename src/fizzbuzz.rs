pub fn fizz_buzz(input: Option<u64>) {
    let limit = match input {
        Some(value) => value,
        None => u64::MAX,
    };

    for i in 1..limit {
        if i % 15 == 0 {
            println!("FizzBuzz");
        }
        else if i % 3 == 0 {
            println!("Fizz");
        }
        else if i % 5 == 0 {
            println!("Buzz");
        }
        else {
            println!("{}", i);
        }
    }
}
