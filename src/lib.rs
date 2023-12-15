pub mod test_utils;
pub mod days;

// use mimalloc::MiMalloc;
// #[global_allocator]
// static GLOBAL: MiMalloc = MiMalloc;

#[cfg(test)]
mod tests {
    use std::time::Instant;
    use super::*;


    #[test]
    fn day1p1() {
        let puzzle = test_utils::read_file("./test_data/day_1_e.txt").unwrap();
        let result = days::day_1::part_1(puzzle);
        assert_eq!(result, 142);

        let puzzle = test_utils::read_file("./test_data/day_1_p.txt").unwrap();
        let result = days::day_1::part_1(puzzle);
        assert_eq!(result, 55447);
    }

    #[test]
    fn day1p2() {
        let puzzle = test_utils::read_file("./test_data/day_1_e2.txt").unwrap();
        let result = days::day_1::part_2(puzzle);
        assert_eq!(result, 281);

        let puzzle = test_utils::read_file("./test_data/day_1_p.txt").unwrap();
        let result = days::day_1::part_2(puzzle);
        assert_eq!(result, 54706);
    }

    #[test]
    fn day2p1() {
        let puzzle = test_utils::read_file("./test_data/day_2_e.txt").unwrap();
        let result = days::day_2::part_1(puzzle);
        assert_eq!(result, 8);

        let puzzle = test_utils::read_file("./test_data/day_2_p.txt").unwrap();
        let result = days::day_2::part_1(puzzle);
        assert_eq!(result, 2541);
    }

    #[test]
    fn day2p2() {
        let pzzl = test_utils::read_file_as_string("./test_data/day_2_e.txt").unwrap();
        let puzzle = pzzl.lines().collect();
        let result = days::day_2::part_2(puzzle);
        assert_eq!(result, 2286);

        let pzzl = test_utils::read_file_as_string("./test_data/day_2_p.txt").unwrap();
        let puzzle = pzzl.lines().collect();
        let result = days::day_2::part_2(puzzle);
        assert_eq!(result, 66016);
    }

    #[test]
    fn day3p1() {
        let pzzl = test_utils::read_file_as_string("./test_data/day_3_e.txt").unwrap();
        let puzzle = pzzl.lines().collect();
        let result = days::day_3::part_1(puzzle);
        assert_eq!(result, 4361);

        let pzzl = test_utils::read_file_as_string("./test_data/day_3_p.txt").unwrap();
        let puzzle = pzzl.lines().collect();
        let result = days::day_3::part_1(puzzle);
        assert_eq!(result, 525911);
    }

    #[test]
    fn day3p2() {
        let pzzl = test_utils::read_file_as_string("./test_data/day_3_e.txt").unwrap();
        let puzzle = pzzl.lines().collect();
        let result = days::day_3::part_2(puzzle);
        assert_eq!(result, 467835);

        let pzzl = test_utils::read_file_as_string("./test_data/day_3_p.txt").unwrap();
        let puzzle = pzzl.lines().collect();
        let result = days::day_3::part_2(puzzle);
        assert_eq!(result, 75805607);
    }

    #[test]
    fn day4p1() {
        let pzzl = test_utils::read_file_as_string("./test_data/day_4_e.txt").unwrap();
        let result = days::day_4::part_1(pzzl.as_str());
        assert_eq!(result, 13);

        let pzzl = test_utils::read_file_as_string("./test_data/day_4_p.txt").unwrap();
        let result = days::day_4::part_1(pzzl.as_str());
        assert_eq!(result, 25651);
    }

    #[test]
    fn day4p2() {
        let pzzl = test_utils::read_file_as_string("./test_data/day_4_e.txt").unwrap();
        let result = days::day_4::part_2(pzzl.as_str());
        assert_eq!(result, 30);

        let pzzl = test_utils::read_file_as_string("./test_data/day_4_p.txt").unwrap();
        let result = days::day_4::part_2(pzzl.as_str());
        assert_eq!(result, 19499881);
    }

    #[test]
    fn day5p1() {
        let pzzl = test_utils::read_file_as_string("./test_data/day_5_e.txt").unwrap();
        let result = days::day_5::part_1(pzzl.as_str());
        assert_eq!(result, 35);

        let pzzl = test_utils::read_file_as_string("./test_data/day_5_p.txt").unwrap();
        let result = days::day_5::part_1(pzzl.as_str());
        // println!("{result}");
        assert_eq!(result, 379811651);
    }

    #[test]
    fn day5p2() {
        let pzzl = test_utils::read_file_as_string("./test_data/day_5_e.txt").unwrap();
        let result = days::day_5::part_2(pzzl.as_str());
        assert_eq!(result, 46);

        let pzzl = test_utils::read_file_as_string("./test_data/day_5_p.txt").unwrap();
        let result = days::day_5::part_2(pzzl.as_str());
        // println!("{result}");
        assert_eq!(result, 27992443);
    }

    #[test]
    fn day6p1() {
        let pzzl = test_utils::read_file_as_string("./test_data/day_6_e.txt").unwrap();
        let result = days::day_6::part_1(pzzl.as_str());
        assert_eq!(result, 288);

        let pzzl = test_utils::read_file_as_string("./test_data/day_6_p.txt").unwrap();
        let result = days::day_6::part_1(pzzl.as_str());
        // println!("{result}");
        assert_eq!(result, 140220);
    }

    #[test]
    fn day6p2() {
        let pzzl = test_utils::read_file_as_string("./test_data/day_6_e.txt").unwrap();
        let result = days::day_6::part_2(pzzl.as_str());
        assert_eq!(result, 71503);

        let pzzl = test_utils::read_file_as_string("./test_data/day_6_p.txt").unwrap();
        let result = days::day_6::part_2(pzzl.as_str());
        // println!("{result}");
        assert_eq!(result, 39570185);
    }

    #[test]
    fn day7p1() {
        let pzzl = test_utils::read_file_as_string("./test_data/day_7_e.txt").unwrap();
        let result = days::day_7::part_1(pzzl.as_str());
        assert_eq!(result, 6440);

        let pzzl = test_utils::read_file_as_string("./test_data/day_7_p.txt").unwrap();
        let result = days::day_7::part_1(pzzl.as_str());
        // println!("{result}");
        assert_eq!(result, 251927063);
    }

    #[test]
    fn day7p2() {
        let pzzl = test_utils::read_file_as_string("./test_data/day_7_e.txt").unwrap();
        let result = days::day_7::part_2(pzzl.as_str());
        assert_eq!(result, 5905);

        let pzzl = test_utils::read_file_as_string("./test_data/day_7_p.txt").unwrap();
        let result = days::day_7::part_2(pzzl.as_str());
        // println!("{result}");
        assert_eq!(result, 255632664);
    }

    #[test]
    fn day8p1() {
        let pzzl = test_utils::read_file_as_string("./test_data/day_8_e1.txt").unwrap();
        let result = days::day_8::part_1(pzzl.as_str());
        assert_eq!(result, 2);

        let pzzl = test_utils::read_file_as_string("./test_data/day_8_e2.txt").unwrap();
        let result = days::day_8::part_1(pzzl.as_str());
        assert_eq!(result, 6);

        let pzzl = test_utils::read_file_as_string("./test_data/day_8_p.txt").unwrap();
        let result = days::day_8::part_1(pzzl.as_str());
        // println!("{result}");
        assert_eq!(result, 21883);
    }

    #[test]
    fn day8p2() {
        let pzzl = test_utils::read_file_as_string("./test_data/day_8_e3.txt").unwrap();
        let result = days::day_8::part_2(pzzl.as_str());
        assert_eq!(result, 6);

        let pzzl = test_utils::read_file_as_string("./test_data/day_8_p.txt").unwrap();
        let result = days::day_8::part_2(pzzl.as_str());
        // println!("{result}");
        assert_eq!(result, 12833235391111);
    }

    #[test]
    fn day9p1() {
        let pzzl = test_utils::read_file_as_string("./test_data/day_9_e.txt").unwrap();
        let result = days::day_9::part_1(pzzl.as_str());
        assert_eq!(result, 114);

        let pzzl = test_utils::read_file_as_string("./test_data/day_9_p.txt").unwrap();
        let result = days::day_9::part_1(pzzl.as_str());
        // println!("{result}");
        assert_eq!(result, 1789635132);
    }

    #[test]
    fn day9p2() {
        let pzzl = test_utils::read_file_as_string("./test_data/day_9_e.txt").unwrap();
        let result = days::day_9::part_2(pzzl.as_str());
        assert_eq!(result, 2);

        let pzzl = test_utils::read_file_as_string("./test_data/day_9_p.txt").unwrap();
        let result = days::day_9::part_2(pzzl.as_str());
        // println!("{result}");
        assert_eq!(result, 913);
    }

    #[test]
    fn day10p1() {
        let pzzl = test_utils::read_file_as_string("./test_data/day_10_e1.txt").unwrap();
        let result = days::day_10::part_1(pzzl.as_str());
        assert_eq!(result, 4);

        let pzzl = test_utils::read_file_as_string("./test_data/day_10_e2.txt").unwrap();
        let result = days::day_10::part_1(pzzl.as_str());
        assert_eq!(result, 4);

        let pzzl = test_utils::read_file_as_string("./test_data/day_10_e3.txt").unwrap();
        let result = days::day_10::part_1(pzzl.as_str());
        assert_eq!(result, 8);

        let pzzl = test_utils::read_file_as_string("./test_data/day_10_e4.txt").unwrap();
        let result = days::day_10::part_1(pzzl.as_str());
        assert_eq!(result, 8);

        let pzzl = test_utils::read_file_as_string("./test_data/day_10_p.txt").unwrap();
        let result = days::day_10::part_1(pzzl.as_str());
        // println!("{result}");
        assert_eq!(result, 6968);
    }

    #[test]
    fn day10p2() {
        let pzzl = test_utils::read_file_as_string("./test_data/day_10_e5.txt").unwrap();
        let result = days::day_10::part_2(pzzl.as_str());
        assert_eq!(result, 4);

        let pzzl = test_utils::read_file_as_string("./test_data/day_10_e6.txt").unwrap();
        let result = days::day_10::part_2(pzzl.as_str());
        assert_eq!(result, 8);

        let pzzl = test_utils::read_file_as_string("./test_data/day_10_e7.txt").unwrap();
        let result = days::day_10::part_2(pzzl.as_str());
        assert_eq!(result, 10);

        let pzzl = test_utils::read_file_as_string("./test_data/day_10_p.txt").unwrap();
        let result = days::day_10::part_2(pzzl.as_str());
        println!("{result}");
        // assert_eq!(result, 6968);
    }

    #[test]
    fn day11p1() {
        let pzzl = test_utils::read_file_as_string("./test_data/day_11_e.txt").unwrap();
        let result = days::day_11::calculate(pzzl.as_str(), 2);
        assert_eq!(result, 374);

        let pzzl = test_utils::read_file_as_string("./test_data/day_11_p.txt").unwrap();
        let result = days::day_11::calculate(pzzl.as_str(), 2);
        // println!("{result}");
        assert_eq!(result, 9805264);
    }

    #[test]
    fn day11p2() {
        let pzzl = test_utils::read_file_as_string("./test_data/day_11_e.txt").unwrap();
        let result = days::day_11::calculate(pzzl.as_str(), 10);
        assert_eq!(result, 1030);
        let result = days::day_11::calculate(pzzl.as_str(), 100);
        assert_eq!(result, 8410);

        let pzzl = test_utils::read_file_as_string("./test_data/day_11_p.txt").unwrap();
        let result = days::day_11::calculate(pzzl.as_str(), 1_000_000);
        // println!("{result}");
        assert_eq!(result, 779032247216);
    }

    #[test]
    fn day12p1() {
        let pzzl = test_utils::read_file_as_string("./test_data/day_12_e.txt").unwrap();
        let result = days::day_12::part_1(pzzl.as_str());
        assert_eq!(result, 21);

        let pzzl = test_utils::read_file_as_string("./test_data/day_12_p.txt").unwrap();
        let result = days::day_12::part_1(pzzl.as_str());
        // println!("{result}");
        assert_eq!(result, 8075);
    }

    // #[test]
    // fn day12p2() {
    //     let pzzl = test_utils::read_file_as_string("./test_data/day_12_e.txt").unwrap();
    //     days::day_12::part_2(pzzl.as_str());
    // assert_eq!(result, 525152);


    // let pzzl = test_utils::read_file_as_string("./test_data/day_12_p.txt").unwrap();
    // days::day_12::part_2(pzzl.as_str());
    //println!("{result}");
    // assert_eq!(result, 8075);
    // }

    #[test]
    fn day13p1() {
        let pzzl = test_utils::read_file_as_string("./test_data/day_13_e.txt").unwrap();
        let result = days::day_13::part_1(pzzl.as_str());
        assert_eq!(result, 405);

        let pzzl = test_utils::read_file_as_string("./test_data/day_13_p.txt").unwrap();
        let result = days::day_13::part_1(pzzl.as_str());
        // println!("{result}");
        assert_eq!(result, 30518);
    }

    #[test]
    fn day13p2() {
        let pzzl = test_utils::read_file_as_string("./test_data/day_13_e.txt").unwrap();
        let result = days::day_13::part_2(pzzl.as_str());
        assert_eq!(result, 400);

        let pzzl = test_utils::read_file_as_string("./test_data/day_13_p.txt").unwrap();
        let result = days::day_13::part_2(pzzl.as_str());
        // println!("{result}");
        assert_eq!(result, 36735);
    }

    #[test]
    fn day14p1() {
        let pzzl = test_utils::read_file_as_string("./test_data/day_14_e.txt").unwrap();
        let result = days::day_14::part_1(pzzl.as_str());
        assert_eq!(result, 136);

        let pzzl = test_utils::read_file_as_string("./test_data/day_14_p.txt").unwrap();
        let start = Instant::now();
        let result = days::day_14::part_1(pzzl.as_str());
        println!("Time taken for calculation: {:.2?}", start.elapsed());
        // println!("{result}");
        assert_eq!(result, 110407);
    }

    #[test]
    fn day14p2() {
        let pzzl = test_utils::read_file_as_string("./test_data/day_14_e.txt").unwrap();
        let result = days::day_14::part_2(pzzl.as_str());
        assert_eq!(result, 64);

        let pzzl = test_utils::read_file_as_string("./test_data/day_14_p.txt").unwrap();
        let result = days::day_14::part_2(pzzl.as_str());
        // println!("{result}");
        assert_eq!(result, 87273);
    }

    #[test]
    fn day15p1() {
        let pzzl = test_utils::read_file_as_string("./test_data/day_15_e.txt").unwrap();
        let result = days::day_15::part_1(pzzl.as_str());
        assert_eq!(result, 1320);

        let pzzl = test_utils::read_file_as_string("./test_data/day_15_p.txt").unwrap();
        let result = days::day_15::part_1(pzzl.as_str());
        // println!("{result}");
        assert_eq!(result, 494980);
    }

    #[test]
    fn day15p2() {
        let pzzl = test_utils::read_file_as_string("./test_data/day_15_e.txt").unwrap();
        let result = days::day_15::part_2(pzzl.as_str());
        assert_eq!(result, 145);

        let pzzl = test_utils::read_file_as_string("./test_data/day_15_p.txt").unwrap();
        let result = days::day_15::part_2(pzzl.as_str());
        // println!("{result}");
        assert_eq!(result, 247933);
    }
}

#[cfg(test)]
mod big_bois {
    use super::*;
    use std::time::Instant;

    #[test]
    fn big3p1() {
        let start = Instant::now();
        let pzz = test_utils::read_file_as_string(r"./big_bois/bb_d3.txt").unwrap();
        let puzzle = pzz.lines().collect();
        let time = start.elapsed();

        let start2 = Instant::now();
        let result = days::day_3::part_1(puzzle);
        let time2 = start2.elapsed();

        println!("{}", result);
        println!("Time taken for calculation: {} ms", time2.as_millis());
        println!("Time taken for read: {} ms", time.as_millis());
    }

    #[test]
    fn big4p1() {
        let start = Instant::now();
        let pzzl = test_utils::read_file_as_string(r"./big_bois/bb_d4.txt").unwrap();
        let time = start.elapsed();

        let start2 = Instant::now();
        let result = days::day_4::part_1(pzzl.as_str());
        let time2 = start2.elapsed();

        println!("{}", result);
        println!("Time taken for calculation: {} ms", time2.as_millis());
        println!("Time taken for read: {} ms", time.as_millis());
    }

    #[test]
    fn big4p2() {
        let start = Instant::now();
        let pzzl = test_utils::read_file_as_string(r"./big_bois/bb_d4.txt").unwrap();
        let time = start.elapsed();

        let start2 = Instant::now();
        let result = days::day_4::part_2(pzzl.as_str());
        let time2 = start2.elapsed();

        println!("{}", result);
        println!("Time taken for calculation: {} ms", time2.as_millis());
        println!("Time taken for read: {} ms", time.as_millis());
    }

    #[test]
    fn big7p1() {
        let start = Instant::now();
        let pzzl = test_utils::read_file_as_string(r"./big_bois/bb_d7.txt").unwrap();
        let time = start.elapsed();

        let start2 = Instant::now();
        let result = days::day_7::part_1(pzzl.as_str());
        let time2 = start2.elapsed();

        println!("{}", result);
        println!("Time taken for calculation: {} ms", time2.as_millis());
        println!("Time taken for read: {} ms", time.as_millis());
    }

    #[test]
    fn big7p2() {
        let start = Instant::now();
        let pzzl = test_utils::read_file_as_string(r"./big_bois/bb_d7.txt").unwrap();
        let time = start.elapsed();

        let start2 = Instant::now();
        let result = days::day_7::part_2(pzzl.as_str());
        let time2 = start2.elapsed();

        println!("{}", result);
        println!("Time taken for calculation: {} ms", time2.as_millis());
        println!("Time taken for read: {} ms", time.as_millis());
    }

    #[test]
    fn big9p1() {
        let start = Instant::now();
        let pzzl = test_utils::read_file_as_string(r"./big_bois/bb_d9.txt").unwrap();
        let time = start.elapsed();

        let start2 = Instant::now();
        let result = days::day_9::part_1(pzzl.as_str());
        let time2 = start2.elapsed();

        println!("{}", result);
        println!("Time taken for calculation: {} ms", time2.as_millis());
        println!("Time taken for read: {} ms", time.as_millis());
    }

    #[test]
    fn big9p2() {
        let start = Instant::now();
        let pzzl = test_utils::read_file_as_string(r"./big_bois/bb_d9.txt").unwrap();
        let time = start.elapsed();

        let start2 = Instant::now();
        let result = days::day_9::part_2(pzzl.as_str());
        let time2 = start2.elapsed();

        println!("{}", result);
        println!("Time taken for calculation: {} ms", time2.as_millis());
        println!("Time taken for read: {} ms", time.as_millis());
    }

    #[test]
    fn big10p1() {
        let start = Instant::now();
        let pzzl = test_utils::read_file_as_string(r"./big_bois/bb_d10.txt").unwrap();
        let time = start.elapsed();

        let start2 = Instant::now();
        let result = days::day_10::part_1(pzzl.as_str());
        let time2 = start2.elapsed();

        println!("{}", result);
        println!("Time taken for calculation: {} ms", time2.as_millis());
        println!("Time taken for read: {} ms", time.as_millis());
    }

    #[test]
    fn big11p1() {
        let start = Instant::now();
        let pzzl = test_utils::read_file_as_string(r"./big_bois/bb_d11.txt").unwrap();
        let time = start.elapsed();

        let start2 = Instant::now();
        let result = days::day_11::calculate(pzzl.as_str(), 2);
        let time2 = start2.elapsed();

        println!("{}", result);
        println!("Time taken for calculation: {} ms", time2.as_millis());
        println!("Time taken for read: {} ms", time.as_millis());
    }

    #[test]
    fn bigger11p1() {
        let start = Instant::now();
        let pzzl = test_utils::read_file_as_string(r"./big_bois/berb_d11.txt").unwrap();
        let time = start.elapsed();

        let start2 = Instant::now();
        let result = days::day_11::calculate(pzzl.as_str(), 2);
        let time2 = start2.elapsed();

        println!("{}", result);
        println!("Time taken for calculation: {} ms", time2.as_millis());
        println!("Time taken for read: {} ms", time.as_millis());
    }

    #[test]
    fn bigger11p2() {
        let start = Instant::now();
        let pzzl = test_utils::read_file_as_string(r"./big_bois/berb_d11.txt").unwrap();
        let time = start.elapsed();

        let start2 = Instant::now();
        let result = days::day_11::calculate(pzzl.as_str(), 1_000_000);
        let time2 = start2.elapsed();

        println!("{}", result);
        println!("Time taken for calculation: {:.2?}", time2);
        println!("Time taken for read: {:.2?}", time);
    }

    #[test]
    fn big13p1() {
        let start = Instant::now();
        let pzzl = test_utils::read_file_as_string(r"./big_bois/bb_d13.txt").unwrap();
        let time = start.elapsed();

        let start2 = Instant::now();
        let result = days::day_13::part_1(pzzl.as_str());
        let time2 = start2.elapsed();

        println!("{}", result);
        println!("Time taken for calculation: {:.2?}", time2);
        println!("Time taken for read: {:.2?}", time);
    }

    #[test]
    fn big13p2() {
        let start = Instant::now();
        let pzzl = test_utils::read_file_as_string(r"./big_bois/bb_d13.txt").unwrap();
        let time = start.elapsed();

        let start2 = Instant::now();
        let result = days::day_13::part_2(pzzl.as_str());
        let time2 = start2.elapsed();

        println!("{}", result);
        println!("Time taken for calculation: {:.2?}", time2);
        println!("Time taken for read: {:.2?}", time);
    }

    #[test]
    fn big14p1() {
        let start = Instant::now();
        let pzzl = test_utils::read_file_as_string(r"./big_bois/bb_d14.txt").unwrap();
        let time = start.elapsed();

        let start2 = Instant::now();
        let result = days::day_14::part_1_large(pzzl.as_str());
        let time2 = start2.elapsed();

        println!("{}", result);
        println!("Time taken for calculation: {:.2?}", time2);
        println!("Time taken for read: {:.2?}", time);
    }
}
