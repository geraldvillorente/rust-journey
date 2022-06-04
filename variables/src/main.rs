fn main() {
    // There are two data types in Rust:
    // * Scalar
    // * Compound
    // Rust is a statically typed language which means
    // that it must know the types of variables at compile time 

    // A scalar type represents a single value
    // Rust has 4 primary scalar types:
    // * Intergers
    // * Floating point numbers
    // * Booleans
    // * Characters
    //
    // Also take note that variable should be in snake_case

    // This is the scalar types
    // 
    // Integers 
    // Working with 32 bit
    let int_num = 2147483647; // This is ma maximum number in 32 bit
    println!("This {} number is the biggest number a 32bit can store!", int_num);

    // This is the limitation of 32bit
    // When you attempt to add any number to 2147483647 it will till throw an error
    // let a = 2147483647;
    // let b = a + 1;
    // Printing b will produce an error: attempt to compute `i32::MAX + 1_i32`, which would overflow

    // To fix the overflow error, manually annotate the variable to 64 bit
    let num: i64 = 2147483647;
    let num = num + 1;
    println!("The value of num is {}", num);

    // Floating point
    // By default Rust allocates 64 bits of space
    let price_banana = 2.0; //f64
    println!("The price_banana ({}) variable is a 64 bit variable", price_banana);
    // To explicitely annotate the foloating point to 32 bit
    let price_apple: f32 = 3.0;
    println!("The price_apple ({}) variable is a 32 bit variable", price_apple);

    // Booleans
    // Booleans have two possible values:
    // * True
    // * False
    let t = true; 
    println!("t is {}", t);
    let f: bool = false; // This is how we explicitly set the type of a variable
    println!("f is {}", f);

    // Character type
    // Note that we specify char literals with single quotes, as opposed to string literals, which use double quotes.
    let g = 'G';
    println!("g is {}", g);
    // String literals
    let name = "Gerald";
    println!("My name is {}", name);


    // These are the compound types
    // 
    // Rust has two compound types:
    // * Tuples
    // * Arrays
    // 
    // Here is Tupple
    let tup: (i32, f64, u8) = (100, 2.8, 2);
    let (x, y, z) = tup;
    println!("x is {}, y is {}, z is {}", x, y, z);
    // Accessing the value manually
    let aa = tup.0;
    let bb = tup.1;
    let cc = tup.2;
    println!("aa is {}", aa);
    println!("bb is {}", bb);
    println!("cc is {}", cc);

    // Array
    // Every element must be in the same type
    // Fixed length, once declared array cannot grow or shrink in size
    let months = ["Jan", "Feb", "Mar", "Apr"];
    println!("The first month is {}", months[0]);
}
