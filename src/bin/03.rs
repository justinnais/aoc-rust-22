use std::{collections::HashSet, vec};

struct Group {
    rucksacks: Vec<Vec<char>>,
}

impl Group {
    pub fn new(rucksacks: Vec<&str>) -> Group {
        Group {
            rucksacks: rucksacks.iter().map(|&r| r.chars().collect()).collect(),
        }
    }

    pub fn find_itersects(&self) -> Vec<&char> {
        let intersects = self
            .rucksacks
            .iter()
            .map(|c| c.iter().collect::<HashSet<&char>>())
            .reduce(|a, b| a.intersection(&b).copied().collect());

        match intersects {
            Some(x) => x.into_iter().collect::<Vec<&char>>(),
            None => Vec::new(),
        }
    }
}

fn char_value(c: &char) -> u32 {
    // gross
    let c = *c;
    match c {
        'a'..='z' => (c as u32) - ('a' as u32) + 1,
        'A'..='Z' => (c as u32) - ('A' as u32) + 27,
        _ => unreachable!(),
    }
}

fn get_priorities(rucksacks: Vec<Group>) -> u32 {
    rucksacks
        .iter()
        .map(|r| {
            r.find_itersects()
                .iter()
                .map(|&i| char_value(i))
                .sum::<u32>()
        })
        .sum()
}

pub fn part_one(input: &str) -> Option<u32> {
    let rucksacks: Vec<Group> = input
        .lines()
        .map(|l| {
            let half = l.len() / 2;
            Group::new(vec![&l[..half], &l[half..]])
        })
        .collect();

    Some(get_priorities(rucksacks))
}

pub fn part_two(input: &str) -> Option<u32> {
    let rucksacks: Vec<Group> = input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|chunk| Group::new(chunk.to_vec()))
        .collect();

    Some(get_priorities(rucksacks))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
