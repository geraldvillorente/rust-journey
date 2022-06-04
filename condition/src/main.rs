fn main() {
    // Example #1
    let number = 6;
    if number < 10 {
        println!("Condition is true")
    } else {
        println!("Condition is false")
    }

    // Example #2
    // Here the number 6 is divisble by 3 and 2 but the result only return the "else if number % 3 == 0" condition
    // The reason is Rust will stop once there is a true condition in the if-else if-else block 
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is divisible by either 4, 3, or 2");
    }
}