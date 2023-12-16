use std::io;

fn main() {
    let mut total: u32 = 0;
    let numbers = [('1', "one"), ('2', "two"), ('3', "three"), ('4', "four"), ('5', "five"),
                   ('6', "six"), ('7', "seven"), ('8', "eight"), ('9', "nine")];

    loop {
        let mut line = String::new();
        let mut first_digit: char =  '\0';
        let mut last_digit: char =  '\0';

        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read a line");

        let line = line.trim();

        if line.is_empty() {
            break;
        }
        
        'mainFor: for (index, letter) in line.chars().enumerate() {
            if letter.is_ascii_digit() {
                first_digit = letter;
                break;
            }
            
            for number in numbers {
                if line[index..].starts_with(number.1) {
                    first_digit = number.0;
                    break 'mainFor;
                }
            }
        }

        'mainFor: for (index, letter) in line.chars().rev().enumerate() {
            if letter.is_ascii_digit() {
                last_digit = letter;
                break;
            }

            for number in numbers {
                if line[line.len() - 1 - index..].starts_with(number.1) {
                    last_digit = number.0;
                    break 'mainFor;
                }
            }
        }

        if last_digit == '\0' {
            last_digit = first_digit;
        }

        let num: String = format!("{}{}", first_digit, last_digit);

        let num: u32 = num.parse().expect("Failed to parse a number");

        total += num;
    }

    println!("{total}");
}
