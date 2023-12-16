use std::io;

fn main() {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let mut total_game_id = 0;

    loop {
        let mut line = String::new();

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        
        let mut acceptable = true;


        io::stdin().read_line(&mut line)
            .expect("Failed to read line");

        let line = line.trim();

        if line.is_empty() {
            break;
        }
        
        let index_of_colon = line.find(":").unwrap();
        let (start_line, end_line) = line.split_at(index_of_colon + 1);
        let game_number: i32 = start_line.split(":").collect::<Vec<&str>>()[0].split(" ").collect::<Vec<&str>>()[1].parse().unwrap();

        let line: Vec<&str> = end_line.trim().split(";").collect();    
        
        for str in line.iter() {
            let str: Vec<&str> = str.trim().split(",").collect();

            for i in str {
                let i = i.trim();
                let i: Vec<&str> = i.split(" ").collect();
                
                let balls: i32 = i[0].trim().parse().unwrap();
                let color = i[1].trim();

                if color.eq("red") {
                    red += balls;
                } else if color.eq("green") {
                    green += balls;
                } else if color.eq("blue") {
                    blue += balls;
                }
            }
            
            if (red > max_red) || (green > max_green) || (blue > max_blue) {
                acceptable = false;
                break;
            }

            red = 0;
            green = 0;
            blue = 0;
        }

        if acceptable {
            total_game_id += game_number;
        }
    }

    println!("total_game_id is {total_game_id}");
}