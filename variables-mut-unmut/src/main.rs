fn main() {
    // This is immutable variable and by default
    // it can't be changed throughout the process.
    let number = 38;
    println!("The number is {}", number);

    // This is a mutable variable, you can alter it throughout 
    // the process.
    let mut age = 38;
    println!("The value of age = {}", age);
    age = 40;
    println!("The value of age = {}", age);
}

// You can execute this using the command: cargo run  