pub mod aoc {
    use crate::util::aoc::input_lines;

    use std::collections::VecDeque;

    #[derive(Clone, Copy)]
    struct Instruction {
        diff: i32,
        latency: u32,
    }

    fn parse_instruction(line: String) -> Instruction {
        if line.is_empty() {
            return Instruction{
                diff: 0,
                latency: 0,
            };
        }
        if line == "noop" {
            return Instruction{
                diff: 0,
                latency: 1,
            };
        }
        if let Some(addx_arg) = line.strip_prefix("addx ") {
            return Instruction{
                diff: addx_arg.parse::<i32>().unwrap(),
                latency: 2,
            };
        }
        panic!("invalid instruction: '{}'", line)
    }

    #[allow(dead_code)]
    pub fn day_main_part() -> std::io::Result<()> {

        let mut program: VecDeque<Instruction> = input_lines()
            .map(parse_instruction)
            .filter(|instr| instr.latency > 0)
            .collect();

        let mut current: Option<Instruction> = program.pop_front();
        let mut x: i32 = 1;
        let mut cycle: i32 = 1;
        let mut signal_strength = 0;

        while let Some(instr) = current {
            // print
            let beam_x = (cycle - 1) % 40;
            let sprite_visible = (x - beam_x).abs() < 2;
            let character = if sprite_visible { "#" } else { "." };
            print!("{}", character);
            if beam_x == 39 {
                println!("");
            }

            let next_x = if instr.latency == 1 && instr.diff != 0 {
                // println!("{}: scheduling +{} for after this cycle. X = {}", cycle, instr.diff, x);
                x + instr.diff
            } else {
                x
            };

            // retire/fetch
            current = if instr.latency == 1 {
                program.pop_front()
            } else {
                Some(Instruction{ diff: instr.diff, latency: instr.latency - 1})
            };
            cycle += 1;

            // write back
            x = next_x;

            // sample
            if cycle >= 20 && (cycle - 20) % 40 == 0 {
                // println!("{}: sampling x = {}", cycle, x);
                signal_strength += x * cycle;
            }
        }
        println!("signal strength: {}", signal_strength);
        Ok(())
    }

    #[allow(non_upper_case_globals, dead_code)]
    pub const day_main: fn() -> std::io::Result<()> = day_main_part;
}