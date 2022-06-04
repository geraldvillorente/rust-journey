fn main() {
    // Example #1
    // Without break this loop will not stop
    loop {
        println!("This is a never ending loop! Press cmd+c to terminate the execution.");
        // You can add break here to stop the loop
        break;
    }

    // Example #2
    let mut num = 1;
    while num != 6 {
        println!("{}", num);
        num = num + 1;
    }

    // Example #3
    let numbers = [1, 2, 3, 4, 5]; // An array of 5 elements
    // Iterate the array
    for element in numbers.iter() {
        println!("Element is {}", element);
    }
}
