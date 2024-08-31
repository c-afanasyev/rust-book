pub fn functions_start() {
    another_function_1();
    another_function_2(100);
    print_labeled_measurement(5,'h');
    let x = five();
    println!("The value of x is: {x}");
    let x = plus_one(x);
    println!("The value of x is: {x}");
}

fn another_function_1() {
    println!("another_function_1");
}

fn another_function_2(x: i32) {
    println!("another_function_2: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}")
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}