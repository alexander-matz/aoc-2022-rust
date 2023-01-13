pub mod aoc {

    #[derive(Debug)]
    struct Section {
        start: u32,
        end: u32,
    }

    fn contains(outer: &Section, inner: &Section) -> bool {
        outer.start <= inner.start && outer.end >= inner.end
    }

    fn overlap(a: &Section, b: &Section) -> bool {
        let min = |x, y| if x < y { x } else { y };
        let max = |x, y| if x > y { x } else { y };
        max(a.start, b.start) <= min(a.end, b.end)
    }

    fn parse_line_p1(line: &str) -> Option<(Section, Section)> {
        let (left, right) = line.split_once(",")?;
        let (left_start, left_end) = left.split_once("-")?;
        let (right_start, right_end) = right.split_once("-")?;
        return Some((
            Section{
                start: left_start.parse::<u32>().ok()?,
                end: left_end.parse::<u32>().ok()?,
            },
            Section{
                start: right_start.parse::<u32>().ok()?,
                end: right_end.parse::<u32>().ok()?,
            }
        ))
    }

    #[allow(dead_code)]
    pub fn day_main_part1() {
        let on_line = |line: &str, acc: u32| -> u32 {
            if line.is_empty() {
                return acc;
            }
            let (left, right) = match parse_line_p1(line) {
                None => { return 0 }
                Some((left, right)) => (left, right),
            };
            if contains(&left, &right) || contains(&right, &left) {
                acc + 1
            } else {
                acc
            }
        };

        let on_done = std::convert::identity;

        let result = crate::util::aoc::run_on_input(0, on_line, on_done);
        println!("overlapping sections part 1: {}", result);
    }

    #[allow(dead_code)]
    pub fn day_main_part2() {
        let on_line = |line: &str, acc: u32| -> u32 {
            if line.is_empty() {
                return acc;
            }
            let (left, right) = match parse_line_p1(line) {
                None => { return 0 }
                Some((left, right)) => (left, right),
            };
            if overlap(&left, &right) {
                acc + 1
            } else {
                acc
            }
        };

        let on_done = std::convert::identity;

        let result = crate::util::aoc::run_on_input(0, on_line, on_done);
        println!("overlapping sections part 2: {}", result);
    }

    #[allow(non_upper_case_globals, dead_code)]
    pub const day_main: fn() = day_main_part2;

}