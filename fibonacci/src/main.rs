use std::io;

fn main() {
    println!("Enter a number: ");
    let number = query_user_for_number();

    let result = get_nth_fib(number);

    println!("Result is: {}", result);
}

fn query_user_for_number() -> u32 {
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");

    let number: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => 1,
    };

    return number;
}

fn get_nth_fib(input: u32) -> u32 {
    return match input {
        1 => 0,
        2 => 1,
        _ => get_nth_fib(input - 1) + get_nth_fib(input - 2),
    };
}
