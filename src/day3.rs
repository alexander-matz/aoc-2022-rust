pub mod aoc {

    use std::{io, str::FromStr};

    #[allow(dead_code)]
    fn play_round_part1(elf: char, me: char) -> i32 {
        let elf_code = elf as u32 - 'A' as u32;
        let me_code = me as u32 - 'X' as u32;
        let points_game = match (3 + me_code - elf_code) % 3 {
            0 => 3,
            1 => 6,
            2 => 0,
            _ => panic!("Unexpected inputs: {} and {}", elf, me)
        };
        points_game + me_code as i32 + 1
    }

    mod id {
        const LOWER_COUNT: usize = 'z' as usize - 'a' as usize + 1;
        const UPPER_COUNT: usize = 'Z' as usize - 'A' as usize + 1;
        pub const ALL_COUNT: usize = LOWER_COUNT + UPPER_COUNT;

        pub fn from_char(ch: char) -> u32 {
            if ch >= 'a' && ch <= 'z' {
                return ch as u32 - 'a' as u32
            } else if ch >= 'A' && ch <= 'Z' {
                return (ch as u32 - 'A' as u32) + LOWER_COUNT as u32
            }
            panic!("cannot convert char {} to id", ch)
        }

        #[allow(dead_code)]
        pub fn to_char(id: u32) -> char {
            if id < LOWER_COUNT as u32 {
                return char::from_u32('a' as u32 + id).unwrap()
            } if id < UPPER_COUNT as u32 {
                return char::from_u32('A' as u32 + id - LOWER_COUNT as u32).unwrap()
            }
            panic!("invalid id: {}", id)
        }
    }

    #[allow(dead_code)]
    fn process_line(line: &str) -> u32 {
        let mut letters_seen = [false; id::ALL_COUNT as usize];
        if line.is_empty() {
            0
        } else {
            let compartment_len = line.len()/2;
            let chars_first = &line[0..compartment_len];
            let chars_second = &line[compartment_len..];
            for ch in chars_first.chars() {
                let id = id::from_char(ch);
                letters_seen[id as usize] = true;
            }
            for ch in chars_second.chars() {
                let id = id::from_char(ch);
                if letters_seen[id as usize] {
                    return id + 1
                }
            }
            panic!("No double items in compartments '{}' and '{}'", chars_first, chars_second)
        }
    }

    #[allow(dead_code)]
    fn find_priority_part2(items1: &str, items2: &str, items3: &str) -> u32 {
        let mut letters_seen = [0; id::ALL_COUNT as usize];
        for ch in items1.chars() {
            let idx = id::from_char(ch) as usize;
            letters_seen[idx] = 1;
        }
        for ch in items2.chars() {
            let idx = id::from_char(ch) as usize;
            if letters_seen[idx] == 1 {
                letters_seen[idx] = 2;
            }
        }
        for ch in items3.chars() {
            let id = id::from_char(ch);
            let idx = id as usize;
            if letters_seen[idx] == 2 {
                return id + 1
            }
        }
        panic!("No triple items in '{}', '{}', and '{}'", items1, items2, items3)
    }

    #[allow(dead_code)]
    pub fn day_main_part1() {
        let on_line = |line: &str, acc: u32| -> u32 {
            let mut letters_seen = [false; id::ALL_COUNT as usize];
            if line.is_empty() {
                return acc;
            }
            let compartment_len = line.len()/2;
            let chars_first = &line[0..compartment_len];
            let chars_second = &line[compartment_len..];
            for ch in chars_first.chars() {
                let id = id::from_char(ch);
                letters_seen[id as usize] = true;
            }
            for ch in chars_second.chars() {
                let id = id::from_char(ch);
                if letters_seen[id as usize] {
                    return acc + id + 1;
                }
            }
            panic!("No double items in compartments '{}' and '{}'", chars_first, chars_second)
        };

        let on_done = |acc: u32| -> u32 {
            acc
        };

        let priority_sum = crate::util::aoc::run_on_input(0, on_line, on_done);
        println!("priority sum part 1: {}", priority_sum);
    }

    #[allow(dead_code)]
    pub fn day_main_part2() {
        #[derive(Debug)]
        struct State {
            bag1: Option<String>,
            bag2: Option<String>,
            acc: u32
        }
        let on_line = |line: &str, state: State| -> State {
            if line.is_empty() {
                return state;
            }

            match state {
                State{bag1: None, bag2: None, acc} => State{
                    bag1: Some(String::from_str(line).unwrap()),
                    bag2: None,
                    acc: acc
                },
                State{bag1: Some(bag1), bag2: None, acc} => State{
                    bag1: Some(bag1),
                    bag2: Some(String::from_str(line).unwrap()),
                    acc: acc
                },
                State{bag1: Some(bag1), bag2: Some(bag2), acc} => {
                    let priority = find_priority_part2(&bag1, &bag2, line);
                    return State{
                        bag1: None,
                        bag2: None,
                        acc: acc + priority
                    }
                },
                _ => panic!("Illegal line state: {:?}", state)
            }
        };

        let on_done = |state: State| -> u32 {
            match state {
                State{bag1: None, bag2: None, acc} => acc,
                _ => panic!("illegal final state: {:?}", state)
            }
        };

        let priority_sum = crate::util::aoc::run_on_input(State{bag1: None, bag2: None, acc: 0}, on_line, on_done);
        println!("priority sum part2: {}", priority_sum);
    }

    #[allow(non_upper_case_globals, dead_code)]
    pub const day_main: fn() = day_main_part2;

}