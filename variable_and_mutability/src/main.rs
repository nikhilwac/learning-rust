fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);

    x=6; // This line will cause a compile-time error because x is immutable
    println!("The value of x is: {}", x);

    const THREE_HOURS_IN_SECONDS: u32 = 60*60*3;
    println!("Three hours in seconds: {}", THREE_HOURS_IN_SECONDS);


    let y = 10;

    let y = y + 1;
    
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {}", y);
    }
    println!("The value of y is: {}", y);
}
