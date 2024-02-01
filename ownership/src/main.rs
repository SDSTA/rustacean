fn main() {
    let mut s = String::from("Hello");

    s.push_str(" World!");

    println!("{}", s);

    let s1 = s;

    // println!("{}", s);
    // error because s moved to s1

    println!("{}", s1);

    take_ownership(s1);

    // println!("{}", s1);
    // error because s1 moved to take_ownership's some_string

    let x = 5;
    make_copy(x);

    println!("{}", x);

    let pstr = String::from("Hello World!");

    let (s2, len) = calculate_length(pstr);

    println!("The length of '{}' is {}", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

fn make_copy(some_integer: i32) {
    println!("{}", some_integer);
}