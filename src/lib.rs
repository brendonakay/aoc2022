use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn solve_day_1_part_1() -> i32 {
    let file_path = "./input/2022/day1.txt";
    let mut leader = 0;

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(file_path) {
        let mut total = 0;
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                if ip.is_empty() {
                    if total > leader {
                        leader = total;
                    }
                    total = 0;
                    continue;
                };
                total += ip.to_string().parse::<i32>().unwrap();
            }
        }
    }

    return leader;
}

pub fn solve_day_1_part_2() -> i32 {
    let file_path = "./input/2022/day1.txt";
    let mut totals = Vec::new();

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(file_path) {
        let mut total = 0;
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                if ip.is_empty() {
                    totals.push(total);
                    total = 0;
                    continue;
                };
                total += ip.to_string().parse::<i32>().unwrap();
            }
        }
    }

    totals.sort();

    let leaders = totals[totals.len() - 3..].into_iter().sum();

    return leaders;
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_1_part_1_works() {
        let result = solve_day_1_part_1();
        assert_ne!(result, 0);
        println!("{}", result)
    }

    #[test]
    fn day_1_part_2_works() {
        let result = solve_day_1_part_2();
        assert_ne!(result, 0);
        println!("{:?}", result)
    }
}
