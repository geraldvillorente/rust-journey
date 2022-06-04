fn main() {
    // Calling the function and storing the value to a variable
    let s = sum(5, 10);
    println!("The sum is {}", s);

    let s1 = sum1(10, 20);
    println!("sum1 sum is {}", s1);
}

// This is a new function
// x and y are parameters
// and the function should return a 32 bit interger
fn sum(x:i32, y:i32) -> i32 {
    let add = x + y;

    return add;
}

// Another version of sum that 
// return the x and y directly
// Take note that there is no semi-colon
// Semi-colon is only applicable to a statement
fn sum1(x:i32, y:i32) -> i32 {
    x + y
}
