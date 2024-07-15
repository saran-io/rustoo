// This file is being used to learn about functions and its notes in Rust Language

fn main() {
    println!("This is the first function");
    println!("hello, world!");
    
    // call the second function
    zoop();

    // call the variable function. Pass the variable inside the function
    hoop(5);

    // area measurement function
    plot(50, 'h');

    // square root aread
    areasq(5);
}

fn zoop() {
    println!("This is the second function")
}

fn hoop(x: i32) {
    println! ("This the loop function and the value of x is : {x}")
}

fn plot (x: i32, unit_label: char) {
    println! ("This is the plot and the height is : {x} and the unit label is : {unit_label}")
}

fn areasq (x: i32) {
    let area = x * x;
    println! ("The area of the plot is : {area}")
}