use std::include_str;

fn main() {
    let output: i32 = include_str!("day1.input")
                    .lines()
                    .map(|line| {
                        let mut line = line.replace("one", "o1e");
                        line = line.replace("two", "t2o");
                        line = line.replace("three", "t3e");
                        line = line.replace("four", "f4r");
                        line = line.replace("five", "f5e");
                        line = line.replace("six", "s6x");
                        line = line.replace("seven", "s7n");
                        line = line.replace("eight", "e8t");
                        line = line.replace("nine", "n9e");
                        println!("{line}");
                        let values: Vec<i32> = line.matches(char::is_numeric)
                            .map(|x| x.parse::<i32>().unwrap()).collect();
                        let first = values.first().unwrap(); 
                        let last = values.last().unwrap(); 
                        return 10*first + last;
                    }).sum();

    println!("{output}");
}
