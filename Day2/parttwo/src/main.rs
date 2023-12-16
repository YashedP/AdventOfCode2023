use std::io;

fn main() {
    let mut total_power = 0;

    loop {
        let mut line = String::new();

        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        
        io::stdin().read_line(&mut line)
            .expect("Failed to read line");

        let line = line.trim();

        if line.is_empty() {
            break;
        }
        
        let index_of_colon = line.find(":").unwrap();

        let line: Vec<&str> = line[(index_of_colon + 1)..].trim().split(";").collect();    
        
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
            
            if red > max_red {
                max_red = red;
            }

            if green > max_green {
                max_green = green;
            }

            if blue > max_blue {
                max_blue = blue;
            }

            red = 0;
            green = 0;
            blue = 0;
        }

        let power = max_red * max_green * max_blue;
        total_power += power;
    }

    println!("The total_power is {}", total_power);
}