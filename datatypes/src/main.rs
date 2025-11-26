fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The guessed number is: {}", guess);

    let x: f32 = 2.0;
    println!("The value of x is: {}", x);

    // Numeric operations
    let sum = 5 + 10;
    let diffrence = 95.5 - 4.3;
    let product = 4 * 30;

    let remainder = 43 % 5;

    // Boolean type
    let t = true;
    let f: bool = false;

    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';

    // Compound types: Tuple and Array
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y,z) = tup;
    println!("The value of y is: {}", y);
}
