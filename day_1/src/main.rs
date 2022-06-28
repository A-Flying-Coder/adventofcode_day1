use std::{fs, iter::Sum};

fn main() {

    fn first_challenge(input_file_name: &str) {
        let sonar_input: String = fs::read_to_string(input_file_name).expect("Something went wrong with reading the file");
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
        println!("First Challenge increases = {}", increases);
    }

    fn second_challenge(input_file_name: &str) {
        let sonar_input: String = fs::read_to_string(input_file_name).expect("Something went wrong with reading the file");
        let lines = sonar_input.lines();
        let mut value_one: i32 = -1;
        let mut value_two: i32 = -1;
        let mut value_three: i32 = -1;
        let mut new_value: i32;
        let mut total: i32 = 0;
        let mut new_total: i32;
        let mut sec_increases: i32 = 0;
        for value in lines {
            new_value = value.parse().unwrap();
            if value_one.is_negative() {
                value_one = new_value;
            } else if value_two.is_negative() {
                value_two = new_value;
            } else if value_three.is_negative() {
                value_three = new_value;
                total = value_one + value_two + value_three;
            } else {
                new_total = value_two + value_three + new_value;
                if  new_total > total {
                    sec_increases += 1;
                }
                total = new_total;
                value_two = value_three;
                value_three = new_value;
            } 
        }
        println!("Second Challenge increases = {}", sec_increases);
    }

    first_challenge("input.txt");
    second_challenge("input.txt");
}
