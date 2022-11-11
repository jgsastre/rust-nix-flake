
//use std::env;
use std::fs;

pub fn solve() {

    let file = "input";

    let contents = fs::read_to_string(file).expect("Should have been able to read the file");

    println!("Read the file! {}", contents);
}
