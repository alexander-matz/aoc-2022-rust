pub mod aoc {
    use std::ops::{Add, Sub};
    use std::collections::HashSet;

    use crate::util::aoc::input_lines;

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct Point {
        x: i32,
        y: i32,
    }

    #[derive(Debug)]
    struct Delta {
        xd: i32,
        yd: i32,
    }

    impl <'a, 'b> Add<&'b Delta> for &'a Point {
        type Output = Point;

        fn add(self, rhs: &'b Delta) -> Self::Output {
            Point{
                x: self.x + rhs.xd,
                y: self.y + rhs.yd,
            }
        }
    }

    impl Add<Delta> for Point {
        type Output = Point;

        fn add(self, rhs: Delta) -> Self::Output {
            Point{
                x: (&self).x + (&rhs).xd,
                y: (&self).y + (&rhs).yd,
            }
        }
    }

    impl <'a, 'b> Sub<&'b Point> for &'a Point {
        type Output = Delta;

        fn sub(self, rhs: &'b Point) -> Self::Output {
            Delta{
                xd: self.x - rhs.x,
                yd: self.y - rhs.y,
            }
        }
    }

    fn parse_line(line: &str) -> Delta {
        let (dir, steps) = line.split_once(' ').unwrap();
        let direction = match dir {
            "R" => (1, 0),
            "L" => (-1, 0),
            "U" => (0, -1),
            "D" => (0, 1),
            x => panic!("unexpected direction: {:?}", x)
        };
        let multiplier = steps.parse::<i32>().unwrap();
        Delta{
            xd: direction.0 * multiplier,
            yd: direction.1 * multiplier,
        }
    }

    fn signum(x: i32) -> i32 {
        if x < 0 {
            -1
        } else if x > 0 {
            1
        } else {
            0
        }
    }

    #[allow(dead_code)]
    fn calculate_tail_move(from: &Point, to: &Point) -> Option<Delta> {
        let dist = to - from;
        if dist.xd.abs() < 2 && dist.yd.abs() < 2 {
            return None;
        }
        Some(Delta{
            xd: signum(dist.xd),
            yd: signum(dist.yd),
        })
    }

    #[allow(dead_code)]
    fn calculate_head_move(from: &Point, to: &Point) -> Option<Delta> {
        let dist = to - from;
        if dist.xd == 0 && dist.yd == 0 {
            return None;
        }
        Some(Delta{
            xd: signum(dist.xd),
            yd: signum(dist.yd),
        })
    }

    fn simulate_move<const KNOT_COUNT: usize>(knots: &mut [Point; KNOT_COUNT], inst: &Delta) -> Vec<Point> {
        assert!(KNOT_COUNT > 1);

        let mut tail_positions: Vec<Point> = Vec::new();
        let head_goal = &knots[0] + inst;
        while let Some(head_step) = calculate_head_move(&knots[0], &head_goal) {
            let new_head = &knots[0] + &head_step;
            // println!("  moved knot [{}] {:?} by {:?} to {:?}", 0, knots[0], head_step, new_head);
            knots[0] = new_head;
            for i in 1..KNOT_COUNT {
                if let Some(knot_step) = calculate_tail_move(&knots[i], &knots[i-1]) {
                    let new_knot = &knots[i] + &knot_step;
                    // println!("  moved knot [{}] {:?} by {:?} to {:?}", i, knots[i], head_step, new_head);
                    if i == KNOT_COUNT - 1 {
                        tail_positions.push(new_knot.clone());
                    }
                    knots[i] = new_knot;
                }
            }
        }
        tail_positions
    }

    #[allow(dead_code)]
    pub fn day_main_part() -> std::io::Result<()> {
        let mut tail_positions_part1: HashSet<Point> = HashSet::new();
        let mut knots_part1 = [(); 2].map(|_| Point{ x: 0, y: 0});
        tail_positions_part1.insert(Point{ x: 0, y: 0 });

        let mut tail_positions_part2: HashSet<Point> = HashSet::new();
        let mut knots_part2 = [(); 10].map(|_| Point{ x: 0, y: 0});
        tail_positions_part2.insert(Point{ x: 0, y: 0 });

        for line in input_lines() {
            if line.is_empty() {
                continue;
            }
            let total_move = parse_line(&line);
            for point in simulate_move(&mut knots_part1, &total_move) {
                tail_positions_part1.insert(point);
            }

            for point in simulate_move(&mut knots_part2, &total_move) {
                tail_positions_part2.insert(point);
            }
        }
        println!("total positions visited by tail 1: {}", tail_positions_part1.len());
        println!("total positions visited by tail 2: {}", tail_positions_part2.len());
        Ok(())
    }

    #[allow(non_upper_case_globals, dead_code)]
    pub const day_main: fn() -> std::io::Result<()> = day_main_part;
}