fn main() {
    println!("Hello, world!");
    another_function(five());
    print_labeled_measurement(plus_one(four()), 'h');
    print_labeled_measurement(plus_one(five()), 'h');
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn four() -> i32 {
    4
}

fn plus_one(x: i32) -> i32 {
    x + 1
}