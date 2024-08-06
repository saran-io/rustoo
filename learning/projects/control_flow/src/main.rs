// this program is to teach out about control flow in rust language
// get an input variables 

fn main() {

    let number = 3;
    
    // if clause check 
    
    if number < 10 {
        println! ("The number is smaller than 10");
    } else {
        println! ("The number is greater than 10");
    }


    // else if clause
    let x = 6;

    if x % 4 == 0 {
        println! ("The number is divisible by 4");
    } else if x % 3 == 0 {
        println! ("The number is divisible by 3");
    } else if x % 2 == 0 {
        println! ("The number is divisible by 2");
    } else {
        println! ("The number is not divisible by 4, 3, or 2");
    }

    // using if in a let statement

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println! ("The value of the number is {}", number);

}