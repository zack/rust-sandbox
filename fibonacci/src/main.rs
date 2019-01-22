use std::io;

fn main() {
    println!("Enter a number: ");
    let number = query_user_for_number();

    let result = fib(number);

    println!("Result is: {}", result);
}

fn query_user_for_number() -> u64 {
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");

    let number: u64 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => 1,
    };

    return number;
}

fn fib(number: u64) -> u64 {
    let result = match number {
        0 | 1 => 1,
        _ => fib_helper(number, 1),
    };

    return result;
}

fn fib_helper(number: u64, total: u64) -> u64 {
    match number {
        0 | 1 => return total,
        _ => return fib_helper(number - 1, total * number),
    };
}
