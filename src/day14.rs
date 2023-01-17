#[allow(dead_code)]
pub mod aoc {

    use crate::grid::{Grid, Point, Vector};
    use crate::parser::Parser;
    use crate::util::input_lines;

    #[derive(Debug, Clone, Copy, PartialEq)]
    enum Mat {
        Air,
        Rock,
        Sand,
    }

    const SAND_INGRESS_X: i64 = 500;

    type Path = Vec<Point>;

    impl std::fmt::Display for Mat {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Mat::Air => write!(f, "."),
                Mat::Rock => write!(f, "#"),
                Mat::Sand => write!(f, "o"),
            }
        }
    }

    fn make_parser() -> std::rc::Rc<dyn Parser> {
        use crate::parser::*;

        make_list(
            make_seq(vec![
                make_capture(make_number()),
                make_lit(","),
                make_capture(make_number())
            ]),
            make_lit(" -> ")
        )
    }

    fn read_paths() -> Vec<Path> {
        let parser = make_parser();
        let mut paths: Vec<Path> = Vec::new();

        for line in input_lines() {
            if line.is_empty() { continue; }
            let (rest, capture) = parser.parse(&line, line.chars()).unwrap();
            assert!(rest.as_str() == "");
            let mut path = Vec::new();
            for point in capture.as_many() {
                let coordinates = point.as_many();
                path.push(
                    Point{
                        x: coordinates[0].as_one().parse().unwrap(),
                        y: coordinates[1].as_one().parse().unwrap(),
                    }
                )
            }
            paths.push(path);
        }
        paths
    }

    fn max_depth(paths: &Vec<Path>) -> i64 {
        *paths.iter().map(
            |path| path.iter().map(|Point{ x: _, y }| y).max().unwrap()
        ).max().unwrap()
    }

    fn grid_from_paths(paths: &Vec<Path>, width: i64, height: i64) -> Grid<Mat> {
        let mut grid = Grid::new(width, height, Mat::Air);

        for path in paths {
            let mut from = None as Option<Point>;
            for point in path {
                from = match from {
                    None => {
                        grid.set(point, Mat::Rock);
                        Some(point.clone())
                    },
                    Some(from) => {
                        let dir = (point - &from).signum();
                        assert!(dir.l1_norm() == 1);
                        let mut step = &from + &dir;
                        while &step != point {
                            grid.set(&step, Mat::Rock);
                            step = &step + &dir;
                        }
                        grid.set(&step, Mat::Rock);
                        Some(point.clone())
                    }
                };
            }
        }

        grid
    }

    fn add_sand(grid: &mut Grid<Mat>, ingress: &Point) -> bool {
        let mut position = ingress.clone();

        if grid.get(&position) == Mat::Sand {
            return false;
        }

        const DOWN: Vector = Vector{ xd: 0, yd: 1 };
        const LEFT: Vector = Vector{ xd: -1, yd: 1 };
        const RIGHT: Vector = Vector{ xd: 1, yd: 1 };

        fn is_viable(grid: &Grid<Mat>, from: &Point, step: &Vector) -> bool {
            let new_pos = from + step;
            if ! grid.is_in_bounds(&new_pos) {
                return true
            }
            grid.get(&new_pos) == Mat::Air
        }

        loop {
            let move_to = {
                if is_viable(grid, &position, &DOWN) {
                    Some(&position + &DOWN)
                } else if is_viable(grid, &position, &LEFT) {
                    Some(&position + &LEFT)
                } else if is_viable(grid, &position, &RIGHT) {
                    Some(&position + &RIGHT)
                } else {
                    None
                }
            };
            match move_to {
                None => break,
                Some(move_to) => {
                    if ! grid.is_in_bounds(&move_to) {
                        return false;
                    }
                    position = move_to
                }
            }
        }

        grid.set(&position, Mat::Sand);
        true
    }

    #[allow(dead_code)]
    pub fn day_main() {
        let paths = read_paths();
        let max_depth = max_depth(&paths);

        // paths.iter().map(|path| println!("{:?}", path)).count();
        // grid.dump_part(&Point{ x: 490, y: 0 }, &Direction{ xd: 20, yd: 10 });

        // let mut grid = Grid::new(SAND_INGRESS_X * 2, max_depth + 1, Mat::Air);

        {
            let mut grid = grid_from_paths(&paths, SAND_INGRESS_X * 2, max_depth + 1);

            let sand_ingress = Point{ x: SAND_INGRESS_X, y: 0 };

            let mut sand_grain_count = 0;
            while add_sand(&mut grid, &sand_ingress) {
                sand_grain_count += 1;
            }
            println!("Able to add {} sand grains without floor", sand_grain_count)
        }

        {
            let mut paths_with_floor = paths.clone();
            paths_with_floor.push(vec![
                Point{ x: 0, y: max_depth + 2 },
                Point{ x: SAND_INGRESS_X * 2, y: max_depth + 2 }
            ]);

            let mut grid = grid_from_paths(&paths_with_floor, SAND_INGRESS_X * 2 + 1, max_depth + 3);

            let sand_ingress = Point{ x: SAND_INGRESS_X, y: 0 };

            let mut sand_grain_count = 0;
            while add_sand(&mut grid, &sand_ingress) {
                sand_grain_count += 1;
            }
            println!("Able to add {} sand grains with floor", sand_grain_count)
        }
    }
}