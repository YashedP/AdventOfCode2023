use std::io;

fn main() {
    let mut total = 0;
    let mut card: i32 = 0;
    let mut cards: Vec<(i32, Vec<i32>)> = Vec::new();
    
    loop {
        let mut line = String::new();

        io::stdin().read_line(&mut line)
            .expect("Failed to read line");

        if line.len() == 0 {
            break;
        }

        card += 1;

        if card > cards.len() as i32 {
            cards.push((card, vec![1]));
        } else {
            cards[card as usize - 1].1 = vec![cards[card as usize - 1].1[0] + 1];
        }

        let (left_numbers, right_numbers) = parse_line(line);

        let mut winning_numbers = 0;
        for i in right_numbers {

            if left_numbers.contains(&i) {
                winning_numbers += 1;
            }
        }
        
        if winning_numbers >= 1 {
            for j in (card as usize + 1)..(card as usize + winning_numbers + 1) {
                if cards.len() as i32 >= j as i32 {
                    println!("{} {}", j, cards[j as usize - 1].1[0] + 1);
                    cards[j as usize - 1].1 = vec![cards[j as usize - 1].1[0] + cards[card as usize - 1].1[0]];
                } else {
                    cards.push((j as i32, vec![cards[card as usize - 1].1[0]]));
                }
            }
        }
        println!("{:?}", cards);
    }
    for i in cards {
        println!("{:?}", i.1[0]);
        total += i.1[0];
    }

    println!("{}", total);
}

fn parse_line(line: String) -> (Vec<i32>, Vec<i32>) {
    let line: Vec<&str> = line.trim().split(": ").collect::<Vec<&str>>()[1].split(" | ").collect::<Vec<&str>>();

    let left_numbers_str = line[0].split_whitespace().collect::<Vec<&str>>();
    let right_numbers_str = line[1].split_whitespace().collect::<Vec<&str>>();

    let mut left_numbers: Vec<i32> = Vec::new();
    let mut right_numbers: Vec<i32> = Vec::new();

    for i in left_numbers_str {
        if i != "" {
            left_numbers.push(i.parse::<i32>().unwrap());
        }
    }

    for i in right_numbers_str {
        if i != "" {
            right_numbers.push(i.parse::<i32>().unwrap());
        }
    }

    (left_numbers, right_numbers)
}

// fn new_vec(string: &str) -> Vec<char> {
//     let mut vec = Vec::new();
//     vec.append(&mut string.chars().collect::<Vec<char>>());
//     vec
// }