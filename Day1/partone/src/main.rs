use std::io;

fn main() {
    let mut total: u32 = 0;

    loop {
        let mut line = String::new();
        let mut first_digit: char =  '\0';
        let mut last_digit: char =  '\0';

        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read a line");

        if line.trim().is_empty() {
            break;
        }
            
        for letter in line.chars() {
            if letter.is_ascii_digit() {
                first_digit = letter;
                break;
            }
        }

        for letter in line.chars().rev() {
            if letter.is_ascii_digit() {
                last_digit = letter;
                break;
            }
        }

        if last_digit == '\0' {
            last_digit = first_digit;
        }

        let num: String = format!("{}{}", first_digit, last_digit);

        let num: u32 = num.parse().expect("Failed to parse a number");

        println!("{num}");
        total += num;
    }

    println!("{total}");
}
