pub fn fizzBuzz(input: Option<u64>) {
    let limit : u64 ;

    match input {
        Some(value) => {
            limit = value;
        }
        None => {
            limit = u64::MAX;
        }
    }

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
