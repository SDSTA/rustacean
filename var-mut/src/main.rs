use std::io;

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("{}", THREE_HOURS_IN_SECONDS);
    println!("{}", 98_222);
    println!("{}", 0xff);
    println!("{}", 0o77);
    println!("{}", 0b1111_0000);
    println!("{}", b'A');

    // addition
    let sum = 5 + 10;
    println!("sum : {}", sum);

    // subtraction
    let difference = 95.5 - 4.3;
    println!("difference : {}", difference);

    // multiplication
    let product = 4 * 30;
    println!("product : {}", product);

    // division
    let quotient = 56.7 / 32.2;
    println!("quotient : {}", quotient);
    let truncated = -5 / 3; // Results in -1
    println!("truncated : {}", truncated);

    // remainder
    let remainder = 43 % 5;
    println!("remainder : {}", remainder);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of x is: {x}\nThe value of y is: {y}\nThe value of z is: {z}");
    println!("first of tup = {}", tup.0);
    println!("second of tup = {}", tup.1);
    println!("third of tup = {}", tup.2);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Please enter an array index");

    let mut index = String::new();

    io::stdin().read_line(&mut index).expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Index entered was not a number.");

    let element = a[index];
    println!("The value of the element at index {index} is: {element}");
}
