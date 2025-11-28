fn main() {
    println!("Hello, world!");
    another_function();
    just_another_function(5);
    print_labeled_measurement(10, 'h');
}

fn another_function() {
    println!("Athother function.")
}

fn just_another_function(x: i32) {
    println!("Th valuer of x is :{x}")
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is : {value}{unit_label}");
    let y = plus_one(value);
    println!("The value plus one is : {y}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}