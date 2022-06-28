use std::fs;

fn main() {
    // input is a list of numbers
    // we need to determine how many times the number is an increase on the previous number
    let sonar_input: String = fs::read_to_string("input.txt").expect("Something went wrong with reading the file");
    let lines = sonar_input.lines();
    let mut prev_value: i32 = 0;
    let mut new_value: i32;
    let mut increases: i32 = -1;
    for value in lines {
        new_value = value.parse().unwrap();
        if  new_value > prev_value {
            increases += 1;
        }
        prev_value = new_value;
    }
    println!("Increases = {}", increases);
}
