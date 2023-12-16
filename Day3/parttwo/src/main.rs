use std::io;

fn main() {
    let mut prev_line: Vec<char> = Vec::new();
    let mut line: Vec<char>;
    let mut next_line: Vec<char>;

    let mut total: i32 = 0;

    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    let input = input.trim();
    line = new_vec(input);

    loop {
        let mut input: String = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();

        next_line = new_vec(input);

        for (index, c) in line.iter().enumerate() {
            if *c == '*' {
                let mut numbers: Vec<i32> = Vec::new();
                
                // check left
                let number = find_number(index as i32 - 1, &line);
                if number != -1 {
                    numbers.push(number);
                }

                // check right
                let number = find_number(index as i32 + 1, &line);
                if number != -1 {
                    numbers.push(number);
                }

                // check top middle, if empty then check top left and top right
                if !prev_line.is_empty() {
                    if prev_line[index].is_numeric() {
                        let number = find_number(index as i32, &prev_line);
                        if number != -1 {
                            numbers.push(number);
                        }
                    } else {
                        let number = find_number(index as i32 - 1, &prev_line);
                        if number != -1 {
                            numbers.push(number);
                        }

                        let number = find_number(index as i32 + 1, &prev_line);
                        if number != -1 {
                            numbers.push(number);
                        }
                    }
                }

                // check bottom middle, if empty then check bottom left and bottom right
                if !next_line.is_empty() {
                    if next_line[index].is_numeric() {
                        let number = find_number(index as i32, &next_line);
                        if number != -1 {
                            numbers.push(number);
                        }
                    } else {
                        let number = find_number(index as i32 - 1, &next_line);
                        if number != -1 {
                            numbers.push(number);
                        }

                        let number = find_number(index as i32 + 1, &next_line);
                        if number != -1 {
                            numbers.push(number);
                        }
                    }
                }

                // if there are 2 numbers, find the gear ratio by multiplying and adding to the total
                if numbers.len() == 2 {
                    println!("numbers[0] is {} and numbers[1] is {}", numbers[0], numbers[1]);
                    total += numbers[0] * numbers[1];
                }
            }
        }

        prev_line = line;
        line = next_line;

        if line.is_empty() {
            break;
        }
    }

    println!("Total is {total}");
}

fn new_vec(string: &str) -> Vec<char> {
    let mut vec: Vec<char> = Vec::new();
    vec.append(&mut string.chars().collect::<Vec<char>>());
    vec
}

/* Go all the way to the left of the number, and take note of the starting index, then go to the 
   right until no more and then keep track of the ending number then parse the number */
fn find_number(index: i32, vector: &Vec<char>) -> i32 {
    let mut num = -1;
    let mut start_index = index;
    
    if start_index == vector.len() as i32 || !vector[start_index as usize].is_numeric() {
        return num;
    }

    // Find the start of the number
    loop {
        // if start_index == 0 {
        if start_index == -1 {
            break;
        }

        if !vector[start_index as usize].is_numeric() {
            start_index += 1;
            break;
        }

        start_index -= 1;
    }

    // Find the end of the number
    let mut end_index = start_index;
    loop {
        if end_index == vector.len() as i32 {
            break;
        }

        if !vector[end_index as usize].is_numeric() {
            break;
        }

        end_index += 1;
    }

    // Parse the number
    let mut string = String::new();
    for i in start_index..end_index {
        string.push(vector[i as usize]);
    }

    if string.len() > 0 {
        num = string.parse::<i32>().unwrap();   
    }

    num
}