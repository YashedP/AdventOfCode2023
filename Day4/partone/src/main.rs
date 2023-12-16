use std::io;

fn main() {
    let mut total = 0;

    loop {
        let mut line = String::new();

        io::stdin().read_line(&mut line)
            .expect("Failed to read line");

        if line.len() == 0 {
            break;
        }

        let (left_numbers, right_numbers) = parse_line(line);

        let mut winning_numbers = 0;
        for i in right_numbers {

            if left_numbers.contains(&i) {
                winning_numbers += 1;
            }

        }
        
        if winning_numbers >= 1 {
            let mut points = 1;    
            
            for _j in 2..(winning_numbers + 1) {
                points *= 2;
            }

            println!("{} points for {} winning numbers", points, winning_numbers);
            total += points;
        }
}

    println!("{}", total);
}

fn parse_line(line: String) -> (Vec<i32>, Vec<i32>) {
    let line: Vec<&str> = line.trim().split(": ").collect::<Vec<&str>>()[1].split(" | ").collect::<Vec<&str>>();

    let left_numbers_str = line[0].split(" ").collect::<Vec<&str>>();
    let right_numbers_str = line[1].split(" ").collect::<Vec<&str>>();

    let mut left_numbers: Vec<i32> = Vec::new();
    let mut right_numbers: Vec<i32> = Vec::new();

    for i in left_numbers_str {
        if i != "" {
            left_numbers.push(i.trim().parse::<i32>().unwrap());
        }
    }

    for i in right_numbers_str {
        if i != "" {
            right_numbers.push(i.trim().parse::<i32>().unwrap());
        }
    }

    (left_numbers, right_numbers)
}

// fn new_vec(string: &str) -> Vec<char> {
//     let mut vec = Vec::new();
//     vec.append(&mut string.chars().collect::<Vec<char>>());
//     vec
// }