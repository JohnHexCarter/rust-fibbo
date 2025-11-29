use std::io;

fn main() {
    println!("Let's calculate a Fibonacci number!");

    let mut fib_minus_one = 1;
    let mut fib_minus_two: u64;
    let mut current_fib = 1;
    let mut fib_index = 2;

    let desired_index = grab_index();

    println!("Calculating the {desired_index}th value of Fibonacci sequence");

    if desired_index > 2 {
        while desired_index > fib_index {
            fib_minus_two = fib_minus_one;
            fib_minus_one = current_fib;

            current_fib = fib_minus_one + fib_minus_two;

            fib_index += 1;
        };
    };

    println!("Value: {current_fib}");
}

fn grab_index() -> u32 {
    let desired_index: u32 = loop {
        println!("Please enter an integer (max of 40)");

        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        let user_input: u32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("not a number");
                continue
            },
        };

        if user_input > 40 {
          println!("too big");
          continue;
        } else {
          break user_input;
        };
    };

    desired_index
}
