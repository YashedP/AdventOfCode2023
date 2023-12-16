use std::io;

fn main() {
    let mut prev_line = String::new();
    let mut line = String::new();
    let mut next_line = String::new();

    let mut total: i32 = 0;

    io::stdin().read_line(&mut line)
        .expect("Failed to read line");

    line = line.trim().to_string();

    'mainLoop: loop {
        io::stdin().read_line(&mut next_line)
            .expect("Failed to read line");

        next_line = next_line.trim().to_string();

        let mut start_index: i32 = -1;
        for (index, c) in line.chars().enumerate() {
            
            if c.is_numeric() {
                if start_index == -1 {
                    start_index = index as i32;
                }
            } 

            let mut line_is_ending = false;

            if index + 1 == line.len() {
                line_is_ending = true;
            }

            if start_index != -1 {
                let mut number_ends = false;

                if line_is_ending {
                    number_ends = true;
                } else {
                    let c = line[index + 1..index + 2].chars().next().unwrap();
                    if !c.is_numeric() {
                        number_ends = true;
                    }
                }
                
                if number_ends {
                    let end_index: i32 = index as i32 + 1;
                    let temp_start_index = start_index;
    
                    println!("{:?}", &line[start_index as usize..index as usize]);

                    let number: i32 = line[start_index as usize..index as usize].parse().unwrap();
                    print!("Number is {}: ", number);
    
                    start_index = -1;
    
                    let mut special_part = false;
    
                    // Check left
                    if temp_start_index != 0 && check_symbol(&line[(temp_start_index - 1) as usize..temp_start_index as usize]) {
                        special_part = true;
                    }
    
                    // Check right
                    if end_index != line.len() as i32 - 1 && check_symbol(&line[(end_index) as usize..(end_index + 1) as usize]) {
                        special_part = true;
                    }
    
                    let mut temp_start_index = temp_start_index - 1;
                    if temp_start_index < 0 {
                        temp_start_index = 0;
                    }
    
                    let mut end_index = end_index + 1;
                    if end_index == line.len() as i32 {
                        end_index = line.len() as i32 - 1;
                    }
    
                    // Check bottom
                    if prev_line != "" && check_symbol(&prev_line[temp_start_index as usize..end_index as usize]) {
                        special_part = true;
                    }
    
                    // Check top
                    if next_line != "" && check_symbol(&next_line[temp_start_index as usize..end_index as usize]) {
                        special_part = true;
                    }
    
                    if special_part {
                        total += number;
                    }
                    
                    println!();    
                }
            }
        }

        if line == "" {
            break 'mainLoop;
        }

        prev_line = line;
        line = next_line.clone();
        next_line = String::new();
    }

    println!("Total is {}", total);
}

fn check_symbol(string: &str) -> bool {
    for c in string.chars() {
        if !(c == '.') {
            if !c.is_numeric() && !c.is_numeric() {
                print!("SUCCESS");
                return true;
            }
        }
    }
    return false;
}