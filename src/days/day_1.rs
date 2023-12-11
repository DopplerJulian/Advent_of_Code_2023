#[allow(unused)]
pub fn part_1(document: Vec<String>) -> u32 {
    document.iter()
        .map(|line| {
            let mut num: u32 = 0;
            num += line.chars().find(|c|c.is_digit(10)).unwrap().to_digit(10).unwrap()*10;
            num += line.chars().rev().find(|c|c.is_digit(10)).unwrap().to_digit(10).unwrap();
            num
        })
        .sum()
}

#[allow(unused)]
pub fn part_2(document: Vec<String>) -> u32 {
    document.iter()
        .map(|line| {
            line_to_number(line)
        })
        .sum()
}

fn line_to_number(line: &str) -> u32{
    let digits = vec!["one", "two", "three", "four", "five", "six", "seven", "eight",
                      "nine", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let mut num: u32 = 0;
    let mut matches: Vec<(usize,&str)> = vec![];

    for d in digits.into_iter(){
        matches.extend(line.match_indices(d))
    }

    matches.sort_by(|a,b| a.0.cmp(&b.0));
    num += match matches.first().unwrap().1 {
        "1" | "one" => 10,
        "2" | "two" => 20,
        "3" | "three" => 30,
        "4" | "four" => 40,
        "5" | "five" => 50,
        "6" | "six" => 60,
        "7" | "seven" => 70,
        "8" | "eight" => 80,
        "9" | "nine" => 90,
        _err_val => panic!("unexpected value: {_err_val}")
    };
    num += match matches.last().unwrap().1 {
        "1" | "one" => 1,
        "2" | "two" => 2,
        "3" | "three" => 3,
        "4" | "four" => 4,
        "5" | "five" => 5,
        "6" | "six" => 6,
        "7" | "seven" => 7,
        "8" | "eight" => 8,
        "9" | "nine" => 9,
        _err_val => panic!("unexpected value: {_err_val}")
    };

    num
}