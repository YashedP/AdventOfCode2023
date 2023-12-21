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
            println!("{}", scanner.read());
            continue;
        }

        // if line[0..1].chars().into_iter().next().unwrap().is_numeric() {
        //     println!("{}", line[0..1].chars().into_iter().next().unwrap().is_numeric());
        // }

        let range: Vec<i64> = line.split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect();

        category_range.push(range);
    }
    categories.push(category_range);

    for i in categories {
        println!("{:?}", i);
    }
}
