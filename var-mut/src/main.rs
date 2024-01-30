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
}
