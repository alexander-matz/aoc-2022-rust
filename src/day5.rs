pub mod aoc {

    use std::collections::VecDeque;

    #[derive(Debug)]
    struct Cargo<const N: usize> {
        stacks: [VecDeque<char>; N],
    }

    impl<const N: usize> Cargo<N> {
        fn new() -> Cargo<N> {
            Cargo{
                stacks: [(); N].map(|_| VecDeque::new()),
            }
        }
    }

    fn str_at(text: &str, idx: usize) -> Option<char> {
        if idx >= text.len() {
            None
        } else {
            Some(text.as_bytes()[idx] as char)
        }
    }

    #[allow(dead_code)]
    fn dump<const N: usize>(cargo: &Cargo<N>) -> () {
        let max_len = cargo.stacks.iter()
            .map(|stack| stack.len())
            .fold(0, |acc, x| std::cmp::max(acc, x));
        for i in 0..max_len {
            let idx = max_len - i - 1;
            for stack in cargo.stacks.iter() {
                if stack.len() > idx {
                    print!("[{}] ", stack[idx]);
                } else {
                    print!("    ");
                }
            }
            println!("");
        }
        for i in 0..cargo.stacks.len() {
            print!(" {}  ", i);
        }
        println!("");
    }

    fn parse_line_stacks<const N: usize>(line: &str, cargo: &mut Cargo<N>) -> bool {
        if str_at(line, 0) != Some('[') {
            return false;
        }
        for stack_idx in 0..N {
            let idx = 1 + 4 * stack_idx;
            match str_at(line, idx) {
                None => {},
                Some(' ') => {},
                Some(ch) => cargo.stacks[stack_idx].push_front(ch)
            }
        }
        true
    }

    #[derive(Debug)]
    struct Command {
        count: usize,
        from: usize,
        to: usize,
    }

    fn parse_line_command(line: &str) -> Option<Command> {
        if !line.starts_with("move") {
            return None;
        }
        let words: Vec<&str> = line.split(' ').collect();
        if words.len() != 6 {
            return None;
        }
        let count = words[1].parse::<i32>().unwrap();
        let from_idx = words[3].parse::<i32>().unwrap() - 1;
        assert!(from_idx >= 0);
        let to_idx = words[5].parse::<i32>().unwrap() - 1;
        assert!(to_idx >= 0);
        Some(Command{
            count: count as usize,
            from: from_idx as usize,
            to: to_idx as usize,
        })
    }

    fn run_line_command_1<const N: usize>(line: &str, cargo: &mut Cargo<N>) -> () {
        let command = match parse_line_command(line) {
            Some(cmd) => cmd,
            None => return
        };
        for _ in 0..command.count {
            match cargo.stacks[command.from].pop_back() {
                Some(ch) => cargo.stacks[command.to].push_back(ch),
                None => ()
            }
        }
    }

    fn run_line_command_2<const N: usize>(line: &str, cargo: &mut Cargo<N>) -> () {
        let command = match parse_line_command(line) {
            Some(cmd) => cmd,
            None => return
        };
        let mut buf = Vec::<char>::new();
        for _ in 0..command.count {
            match cargo.stacks[command.from].pop_back() {
                Some(ch) => buf.push(ch),
                None => ()
            }
        }
        while ! buf.is_empty() {
            match buf.pop() {
                Some(ch) => cargo.stacks[command.to].push_back(ch),
                None => ()
            }
        }
    }

    fn get_top_crates<const N: usize>(cargo: &Cargo<N>) -> String {
        let mut buffer = Vec::<u8>::new();
        buffer.reserve(N);
        for idx in 0..N {
            match cargo.stacks[idx].back() {
                Some(ch) => buffer.push(*ch as u8),
                None => buffer.push(' ' as u8),
            }
        }
        String::from_utf8(buffer).unwrap()
    }

    const NUM_STACKS: usize = 9;

    #[allow(dead_code)]
    pub fn day_main_part1() {
        let on_line = |line: &str, mut cargo: Cargo<NUM_STACKS>| -> Cargo<NUM_STACKS> {
            if line.is_empty() {
                return cargo;
            }
            if parse_line_stacks(line, &mut cargo) {
                return cargo;
            }
            run_line_command_1(line, &mut cargo);
            cargo
        };

        let on_done = std::convert::identity;

        let result = crate::util::aoc::run_on_input(Cargo::<NUM_STACKS>::new(), on_line, on_done);
        dump(&result);
        println!("Top crates part 1: {}", get_top_crates(&result));
    }

    #[allow(dead_code)]
    pub fn day_main_part2() {
        let on_line = |line: &str, mut cargo: Cargo<NUM_STACKS>| -> Cargo<NUM_STACKS> {
            if line.is_empty() {
                return cargo;
            }
            if parse_line_stacks(line, &mut cargo) {
                return cargo;
            }
            run_line_command_2(line, &mut cargo);
            cargo
        };

        let on_done = std::convert::identity;

        let result = crate::util::aoc::run_on_input(Cargo::<NUM_STACKS>::new(), on_line, on_done);
        dump(&result);
        println!("Top crates part 2: {}", get_top_crates(&result));
    }

    #[allow(non_upper_case_globals, dead_code)]
    pub const day_main: fn() = day_main_part2;

}