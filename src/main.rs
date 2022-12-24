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

mod day2 {
    use std::{convert::Infallible, str::FromStr};

    #[derive(Copy, Clone)]
    enum Outcome {
        Lost = 0,
        Draw = 3,
        Win = 6,
    }

    #[derive(Copy, Clone, PartialEq)]
    enum Hand {
        Rock = 1,
        Paper = 2,
        Scissors = 3,
    }

    impl Hand {
        fn against(&self, foe: &Hand) -> Outcome {
            match self {
                s if *s == foe.lose_against() => Outcome::Win,
                s if *s == foe.win_against() => Outcome::Lost,
                _ => Outcome::Draw,
            }
        }

        fn lose_against(&self) -> Hand {
            match self {
                Hand::Rock => Hand::Paper,
                Hand::Paper => Hand::Scissors,
                Hand::Scissors => Hand::Rock,
            }
        }

        fn win_against(&self) -> Hand {
            match self {
                Hand::Rock => Hand::Scissors,
                Hand::Paper => Hand::Rock,
                Hand::Scissors => Hand::Paper,
            }
        }
    }

    impl FromStr for Hand {
        type Err = Infallible;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "A" => Ok(Hand::Rock),
                "B" => Ok(Hand::Paper),
                "C" => Ok(Hand::Scissors),
                // for part1
                "X" => Ok(Hand::Rock),
                "Y" => Ok(Hand::Paper),
                "Z" => Ok(Hand::Scissors),
                _ => panic!("char isn't an Hand"),
            }
        }
    }

    impl FromStr for Outcome {
        type Err = Infallible;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "X" => Ok(Outcome::Lost),
                "Y" => Ok(Outcome::Draw),
                "Z" => Ok(Outcome::Win),
                _ => panic!("char isn't an Outcome"),
            }
        }
    }

    pub fn part_1(input: &str) -> i64 {
        let mut score: i64 = 0;
        for line in input.lines() {
            let mut split = line.split_whitespace();

            let foe: Hand = split.next().unwrap().parse().unwrap();
            let me: Hand = split.next().unwrap().parse().unwrap();

            {
                score += me.against(&foe) as i64 + me as i64;
            }

            print!("{}", me as i64)
        }
        score
    }

    pub fn part_2(input: &str) -> i64 {
        let mut score: i64 = 0;
        for line in input.lines() {
            let mut split = line.split_whitespace();

            let foe: Hand = split.next().unwrap().parse().unwrap();
            let result: Outcome = split.next().unwrap().parse().unwrap();
            let me: Hand = match result {
                Outcome::Lost => foe.win_against(),
                Outcome::Draw => foe,
                Outcome::Win => foe.lose_against(),
            };

            score += result as i64 + me as i64;
        }
        score
    }
}

mod day3 {
    use std::collections::HashSet;

    fn item_value(item: &char) -> i64 {
        if item.is_ascii_lowercase() {
            *item as i64 - ('a' as i64 - 1)
        } else {
            *item as i64 - ('A' as i64 - 1) + 26
        }
    }

    pub fn part_1(input: &str) -> i64 {
        let mut priority_total = 0;

        for rucksack in input.lines() {
            let (compartment1, compartment2) = rucksack.split_at(rucksack.len() / 2);

            let compartment1: HashSet<char> = compartment1.chars().collect();
            let compartment2: HashSet<char> = compartment2.chars().collect();

            let commonitem = compartment1.intersection(&compartment2).next().unwrap();

            priority_total += item_value(commonitem);
        }

        return priority_total;
    }

    pub fn part_2(input: &str) -> i64 {
        let mut priority_total = 0;

        let mut lines = input.lines();

        while let (Some(a), Some(b), Some(c)) = (lines.next(), lines.next(), lines.next()) {
            let a: HashSet<char> = a.chars().collect();
            let b: HashSet<char> = b.chars().collect();
            let c: HashSet<char> = c.chars().collect();

            let badgeitem = a
                .iter()
                .filter(|item| b.contains(item) && c.contains(item))
                .next()
                .unwrap();

            priority_total += item_value(badgeitem);
        }

        return priority_total;
    }
}

aoc_main::main! {
    year 2022;
    day1 => part_1, part_2;
    day2 => part_1, part_2;
    day3 => part_1, part_2;
}
