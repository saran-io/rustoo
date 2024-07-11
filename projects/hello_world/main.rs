// Title: Hello, world!
// Description: A simple hello world program in Rust.
// Created: 2018-12-02
// Author: Saranraj Santhanam   


//fn main() {
//    println!("Hello Saran, Welcome to Rust world!")
//}

fn main() {
    println!("{}",greet("Saran"));
}

fn greet(name: &str) -> String {
    format!("Hello, {}, Welcome to Rust world!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        assert_eq!(greet("Saran"), "Hello, Saran, Welcome to Rust world!")
    }
}
 