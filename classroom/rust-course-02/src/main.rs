fn main() {
    let my_name: &'static str = "Jackson Douglas";
    print!("My name ys: {} and ", my_name);
    let my_name_two: &str = "de Souza \n";
    print!("My two name is: {}", my_name_two);
    // my_name = "Jackson" // This line will cause a compile-time error because `my_name` is immutable.

    let mut mutable_variable: i32 = 1;
    println!("\nThe value of mutable_variable is: {}", mutable_variable);
    mutable_variable = 20; // This is allowed because `mutable_variable` is mutable.
    println!("\nThe value of mutable_variable is: {}", mutable_variable); 

    let x: i32 = 5;
    let y: i32 = 15;
    // x = 10; // This line will cause a compile-time error because `x` is immutable.
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    let sun: i32 = x + y;
    println!("The sum of x and y is: {}", sun);

    let median: i32 = ( x + y ) / 2;
    println!("The median of x and y is: {}", median);

    let median_01: f64 = 85.58;
    let median_02: f64 = 75.00;
    let median_final: f64 = ( median_01 + median_02 ) / 2.0;
    println!("The final median is: {}", median_final);

    let variable01 = false;
    println!("The value of variable01 is: {}", variable01);
    let variable02: bool = true;
    println!("The value of variable02 is: {}", variable02);


}
