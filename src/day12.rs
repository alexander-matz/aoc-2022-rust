#[allow(dead_code)]
pub mod aoc {
    use std::collections::BinaryHeap;
    use std::cmp::Reverse;

    use crate::util::aoc::input_lines;

    struct Grid<T: Copy> {
        data: Vec<T>,
        width: i32,
        height: i32,
    }

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl <T: Copy> Grid<T> {

        fn new(width: i32, height: i32, value: T) -> Grid<T> {
            Grid{
                data: vec![value; (width * height) as usize],
                width: width,
                height: height,
            }
        }

        fn get(&self, p: &Point) -> T {
            assert!(p.x >= 0 && p.x < self.width);
            assert!(p.y >= 0 && p.y < self.height);
            self.data[(p.x + p.y * self.width) as usize]
        }

        fn set(&mut self, p: &Point, value: T) {
            assert!(p.x >= 0 && p.x < self.width);
            assert!(p.y >= 0 && p.y < self.height);
            self.data[(p.x + p.y * self.width) as usize] = value
        }
    }

    impl std::fmt::Debug for Grid<i8> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for row in 0..self.height {
                for col in 0..self.width {
                    let height = self.get(&Point{ x: col, y: row });
                    let char = (height + 'a' as i8) as u8 as char;
                    write!(f, "{}", char)?
                }
                writeln!(f, "")?
            }
            Ok(())
        }
    }

    fn read_grid(input: &Vec<String>) -> (Grid<i8>, Point, Point) {
        assert!(input.len() > 0);
        let cols = input[0].len();
        let rows = input.len();
        let mut grid = Grid::new(cols as i32, rows as i32, 0);
        let mut start: Option<Point> = None;
        let mut end: Option<Point> = None;
        for row in 0..rows {
            let line = &input[row];
            assert!(line.len() == cols);

            for (col, val) in line.char_indices() {
                let pos = Point{ x: col as i32, y: row as i32 };
                let height= match val {
                    'S' => {
                        start = Some(pos.clone());
                        'a'
                    },
                    'E' => {
                        end = Some(pos.clone());
                        'z'
                    },
                    val if val >= 'a' && val <= 'z' => val,
                    val => panic!("Expected either S, E, or a-z, but got '{}'", val)
                } as i8 - 'a' as i8;
                grid.set(&pos, height);
            }
        }
        (grid, start.unwrap(), end.unwrap())
    }

    fn a_star<HeurT, ViableT, DoneT>(grid: &Grid<i8>, start: &Point,
        h: &HeurT, viable: &ViableT, done: &DoneT) -> u32
    where
        HeurT: Fn(&Point) -> f32,
        ViableT: Fn(&Point, &Point) -> bool,
        DoneT: Fn(&Point) -> bool,
    {
        #[derive(PartialEq, PartialOrd, Eq, Ord)]
        struct Candidate {
            cost: i32,
            point: Point,
        }

        let make_candidate = |current_dist: i32, from: &Point| {
            Reverse(Candidate{
                cost: current_dist * 100 + (h(from) * 100.0) as i32,
                point: from.clone()
            })
        };

        let mut frontier: BinaryHeap<Reverse<Candidate>> = BinaryHeap::new();
        let mut distances = Grid::new(grid.width, grid.height, std::u32::MAX);

        distances.set(start, 0);
        frontier.push(make_candidate(0, start));
        while let Some(Reverse(Candidate{ cost: _, point })) = frontier.pop() {
            let current_dist = distances.get(&point);
            let new_dist = current_dist + 1;
            for (xd, yd) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let new_point = Point{ x: point.x + xd, y: point.y + yd };
                if !viable(&point, &new_point) {
                    continue
                }
                if done(&new_point) {
                    return new_dist
                }
                if distances.get(&new_point) <= new_dist { continue };
                distances.set(&new_point, new_dist);
                frontier.push(make_candidate(new_dist as i32, &new_point));
            }
        }
        panic!("Did not find any path!");
    }

    pub fn day_main() -> std::io::Result<()> {
        let lines: Vec<String> = input_lines().collect();
        let (grid, start, end) = read_grid(&lines);

        // println!("{:?}", grid);
        println!("start: {:?}, end: {:?}", start, end);

        {
            let heuristic = |from: &Point| {
                let xd = (from.x - end.x).abs() as f32;
                let yd = (from.y - end.y).abs() as f32;
                (xd * xd + yd * yd).sqrt()
            };

            let viable = |from: &Point, to: &Point| {
                if to.x < 0 || to.x >= grid.width || to.y < 0 || to.y >= grid.height {
                    return false
                }
                let old_height = grid.get(from);
                let new_height = grid.get(to);
                if new_height - old_height > 1 {
                    return false
                }
                true
            };

            let done = |p: &Point| {
                p == &end
            };

            let distance = a_star(&grid, &start, &heuristic, &viable, &done);
            println!("minimum distance forward: {}", distance);
        }

        {
            let heuristic = |from: &Point| {
                let xd = (from.x - end.x).abs() as f32;
                let yd = (from.y - end.y).abs() as f32;
                (xd * xd + yd * yd).sqrt()
            };

            let viable = |from: &Point, to: &Point| {
                if to.x < 0 || to.x >= grid.width || to.y < 0 || to.y >= grid.height {
                    return false
                }
                let old_height = grid.get(from);
                let new_height = grid.get(to);
                if new_height - old_height < -1 {
                    return false
                }
                true
            };

            let done = |p: &Point| {
                grid.get(p) == 0
            };

            let distance = a_star(&grid, &end, &heuristic, &viable, &done);
            println!("minimum distance backward: {}", distance);
        }

        Ok(())
    }
}