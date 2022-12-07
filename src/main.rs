mod day1 {
    pub fn part_1(input: &str) -> i64 {
        let mut max: i64 = 0;
        let mut buffer: i64 = 0;
        for line in input.lines() {
            if line.is_empty() {
                if buffer > max {
                    max = buffer
                }
                buffer = 0;
            } else {
                buffer += line.parse::<i64>().unwrap()
            }
        }
        max
    }

    pub fn part_2(input: &str) -> i64 {
        let topthree: &mut [i64] = &mut [0, 0, 0];
        let mut buffer: i64 = 0;
        for line in input.lines() {
            if line.is_empty() {
                if buffer > topthree[2] {
                    topthree[0] = topthree[1];
                    topthree[1] = topthree[2];
                    topthree[2] = buffer;
                } else if buffer > topthree[1] {
                    topthree[0] = topthree[1];
                    topthree[1] = buffer;
                } else if buffer > topthree[0] {
                    topthree[0] = buffer;
                }
                buffer = 0;
            } else {
                buffer += line.parse::<i64>().unwrap()
            }
        }

        return topthree.iter().sum();
    }
}

// mod day2 {
//     pub fn part_1(input: &str) -> i64 {
//         for line in input.lines() {

//         }
//     }
// }
// mod day3;

aoc_main::main! {
    year 2022;
    day1 => part_1, part_2;
    // day2 => part_1;
}
