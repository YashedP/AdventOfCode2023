mod scanner;

fn main() {
    let mut scanner = scanner::Scanner::new();

    let line = scanner.read();
    let seeds: Vec<i64> = line[6..line.len()].split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    
    let mut categories: Vec<Vec<Vec<i64>>> = Vec::new();
    let mut category_range: Vec<Vec<i64>> = Vec::new();
    
    while scanner.has_next_line() {
        let line = scanner.read();
        
        if line.len() == 0 {
            if category_range.len() > 0 {
                categories.push(category_range);
                category_range = Vec::new();    
            }
            
            scanner.read();
            continue;
        }

        let range: Vec<i64> = line.split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect();

        category_range.push(range);
    }
    categories.push(category_range);

    let mut seeds: Vec<(i64, bool)> = seeds.iter().map(|x| (*x, false)).collect();

    for category in categories.iter() {
        for range in category.iter() {
            let source_start = range[1];
            let dest_start = range[0];
            let length = range[2];
            let source_end = source_start + length - 1;

            for (seed, bool) in seeds.iter_mut() {
                if *seed >= source_start && *seed <= source_end && *bool == false {
                    *seed = *seed - source_start + dest_start;
                    *bool = true;
                }
            }
        }

        for (_, bool) in seeds.iter_mut() {
            *bool = false;
        }
    }

    println!("{}", seeds.iter().map(|x| x.0).collect::<Vec<i64>>().iter().min().unwrap());
}
