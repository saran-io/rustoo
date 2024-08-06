// This code block is to understand how the ownership works in Rust lang
// this is being picked up from the examples

// Rust lang ownership notes
// memory managed via system of ownership
// if rules violated then it won't compile

// stack vs heap
// stack - organized, efficient, push & pop, fixed size data
// heap - managed by memory allocator, allocation/de-allocation, data via pointers

// scope - Range within a program  for which an item is valid


fn main() {

    //Initialize and intro
    println! ("This set of programs is for ownership");

    let _xoo = "hello";

    //memory and allocation examples
    println! ("This following section provides and example for memory and allocation");
    // yoo is valid from the here
    let _yoo = String::from("Hello");
    println! ("Print the string yoo: {_yoo}");
    // scope of yoo is over now


    //variables and data interacting with Move
    println! ("Section : Variables and Data Interacting with Move");

    let a = 5;
    let b = a;
    println! ("Value of the Variable a is {a}");
    println! ("Value of the Variable b is {b}");

    //  string value management in ownership
    println! ("Section - Learning about the string management in Ownership");
    let s1 = String::from("Hello");
    let s2 = s1;
    println! ("Print the string s2 {s2}");
    //println! ("Print the string s1 {s1}");

    // variables and data interactive with clone
    // deep copy in action
    println! ("Section - Learn about clone");
    let s3 = String::from("Rust lang");
    let s4 = s3.clone();

    println! ("Print the values of S3 and S4 : {s3} & {s4}");

}



