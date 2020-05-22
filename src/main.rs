use std::io;

fn main() {

    loop {
        //SOMETHING CHANGE
        println!("Write a number.");
        let stdin = io::stdin(); 
        let number = {
            let mut nums = String::new();
            stdin.read_line(&mut nums)
                .expect("Unable to get str!");
            nums
        };

        if number == "quit" {
            break;
        }

        let number: i32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Unable to parse str as int!");
                continue;
            }
        };
        print!("Answer: ");
        print_binary_number(number);
        println!();
    }
} 

fn print_binary_number(number: i32) -> () {
    let count_numbers: u8 = (number as f64).log2() as u8;

    for i in (0..=count_numbers).rev() {
        print!("{}", if ((1 << i) & number) != 0 {'1'} else {'0'});
    }
}
