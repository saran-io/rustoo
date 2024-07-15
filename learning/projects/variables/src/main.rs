// this is created to learn about variables in rust lang

fn main() {

    //variables explained
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    //shadowing concept

    let y = 5;
    let y = y + 1;
     {
        let y = y * 2;
        println!("The value of Y is: {y}");
     }
    println!("The value of Y is: {y}");


    //shadowing using space example
    let spaces = "    ";
    let spaces = spaces.len();
    println!("The lengh of spsces is: {spaces}")
}
