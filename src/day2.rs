pub mod aoc {

use std::io;

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

#[allow(dead_code)]
fn play_round_part2(elf: char, outcome: char) -> i32 {
    let elf_code = elf as u32 - 'A' as u32;
    let outcome_code = outcome as u32 - 'X' as u32;
    let me_code = (2 + elf_code + outcome_code) % 3;
    let points_game = match (3 + me_code - elf_code) % 3 {
        0 => 3,
        1 => 6,
        2 => 0,
        _ => panic!("Unexpected inputs: {} and {}", elf, outcome)
    };
    points_game + me_code as i32 + 1
}

fn process_line(line: &str) -> i32 {
    if line.is_empty() {
        0
    } else {
        let mut chars = line.chars();
        let elf = chars.next().unwrap();
        assert!("ABC".contains(elf));
        assert!(chars.next().unwrap() == ' ');
        let me = chars.next().unwrap();
        assert!("XYZ".contains(me));
        let points = play_round_part2(elf, me);
        // println!("Playing: {} vs {} -> {}", elf, me, points);
        points
    }
}

pub fn day_main() -> io::Result<()> {
    let mut total_points = 0;
    loop {
        let mut buffer = String::new();
        match io::stdin().read_line(&mut buffer) {
            Ok(0) => {
                println!("Total points: {}", total_points);
                return Ok(())
            },
            Ok(_) => {
                let line = buffer.trim();
                total_points += process_line(line);
            },
            Err(error) => return Err(error)
        }
    }
}

}