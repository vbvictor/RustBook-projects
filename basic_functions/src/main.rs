fn fahrenheit_to_celsius(temp: f64) -> f64 {
    temp * 9.0 / 5.0 + 32.0
}

fn generate_fibonacci(pos: i32) -> i32 {
    if pos == 1 {
        1
    } else if pos == 2 {
        1
    } else {
        generate_fibonacci(pos - 1) + generate_fibonacci(pos - 2)
    }
}

fn main() {
    println!(
        "Hello, world fellow US citizens! Today temperature is {}F",
        fahrenheit_to_celsius(26.0)
    );

    println!("10th Fibonacci number is {}!", generate_fibonacci(10));
}
