#![allow(dead_code)]

use crate::util::input_all;
use crate::grid::{Grid, Dimensions, Point, Vector};

#[derive(Debug, Clone, Copy)]
enum Mat {
    Air,
    Rock,
}

impl std::fmt::Display for Mat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mat::Air => write!(f, "."),
            Mat::Rock => write!(f, "#"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Left = 0,
    Right,
    Up,
    Down,
}

// NOTE: Up is *positive* y delta
impl Direction {
    pub const ALL: [Direction; 4] = [Direction::Left, Direction::Right, Direction::Up, Direction::Down];
    const fn as_vector(&self) -> Vector {
        match self {
            Direction::Left => Vector{ xd: -1, yd: 0 },
            Direction::Right => Vector{ xd: 1, yd: 0 },
            Direction::Up => Vector{ xd: 0, yd: 1 },
            Direction::Down => Vector{ xd: 0, yd: -1 },
        }
    }

    const fn opposite(&self) -> Direction {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }
}

struct FixedVec<T, const N: usize> {
    data: [T; N],
    len: usize
}

// It is *MUCH* easier to just keep the live piece separate instead of immediately
// rendering it to the grid, since it allows to just check all points of the shape
// instead of calculating which pixels moved. I'm leaving this code in because it's
// a fun experiment in constant evaluation in rust.
mod pieces {
    use super::{Direction, Dimensions, Point, FixedVec};

    #[derive(Debug, Clone, Copy)]
    pub enum Piece {
        Horizontal = 0,
        Cross,
        LeftL,
        BigI,
        Block,
    }
    impl Piece {
        pub const ALL: [Piece; 5] = [Piece::Horizontal, Piece::Cross, Piece::LeftL, Piece::BigI, Piece::Block];
    }

    const fn as_points<const N: usize>(pairs: [(i64, i64); N]) -> [Point; N] {
        let mut result: [Point; N] = [Point{ x: 0, y: 0}; N];
        let mut i: usize = 0;
        while i < pairs.len() {
            result[i] = Point{ x: pairs[i].0, y: pairs[i].1 };
            i += 1;
        }
        result
    }

    const BASE: [(Dimensions, &'static [Point]); Piece::ALL.len()] = [
        ( Dimensions{ w: 4, h: 1 }, &as_points([(0, 0), (1, 0), (2, 0), (3, 0)]) ),
        ( Dimensions{ w: 3, h: 3 }, &as_points([(1, 2), (0, 1), (1, 1), (2, 1), (1, 0)]) ),
        ( Dimensions{ w: 3, h: 3 }, &as_points([(2, 2), (2, 1), (0, 0), (1, 0), (2, 0)]) ),
        ( Dimensions{ w: 1, h: 4 }, &as_points([(0, 3), (0, 2), (0, 1), (0, 0)]) ),
        ( Dimensions{ w: 2, h: 2 }, &as_points([(0, 1), (1, 1), (0, 0), (1, 0)]) ),
    ];

    const SHIFTED: [[FixedVec<Point, 4>; Direction::ALL.len()]; Piece::ALL.len()] = [
        shift_data_for_piece(Piece::Horizontal),
        shift_data_for_piece(Piece::Cross),
        shift_data_for_piece(Piece::LeftL),
        shift_data_for_piece(Piece::BigI),
        shift_data_for_piece(Piece::Block),
    ];

    pub fn dims(piece: Piece) -> Dimensions {
        BASE[piece as usize].0
    }

    pub fn points(piece: Piece) -> &'static [Point] {
        BASE[piece as usize].1
    }

    pub fn shifted_points(piece: Piece, shift: Direction) -> &'static [Point] {
        let vec = &SHIFTED[piece as usize][shift as usize];
        &vec.data[0..vec.len]
    }

    const fn is_shifted_point_occluded(points: &'static [Point], point_idx: usize, direction: Direction) -> bool {
        let unshifted = &points[point_idx];
        let shift = direction.as_vector();
        let shifted = Point{
            x: unshifted.x + shift.xd,
            y: unshifted.y + shift.yd,
        };
        let mut reference_idx: usize = 0;
        let mut occluded = false;
        while reference_idx < points.len() {
            let reference = &points[reference_idx];
            if shifted.x == reference.x && shifted.y == reference.y {
                occluded = true;
                break;
            }
            reference_idx += 1;
        }
        occluded
    }

    const fn shift_frontier<const N: usize>(shape: &'static [Point], direction: Direction) -> FixedVec<Point, N> {
        let mut points: [Point; N] = [Point{ x: 0, y: 0 }; N];

        let mut candidate_idx: usize = 0;
        let mut write_idx: usize = 0;
        while candidate_idx < shape.len() {
            let unshifted = &shape[candidate_idx];
            let shift = direction.as_vector();
            let shifted = Point{
                x: unshifted.x + shift.xd,
                y: unshifted.y + shift.yd,
            };
            let mut reference_idx: usize = 0;
            let mut occluded = false;
            while reference_idx < shape.len() {
                let reference = &shape[reference_idx];
                if shifted.x == reference.x && shifted.y == reference.y {
                    occluded = true;
                    break;
                }
                reference_idx += 1;
            }
            if ! occluded {
                assert!(write_idx < N);
                points[write_idx] = shifted;
                write_idx += 1;
            }
            candidate_idx += 1;
        }
        FixedVec{
            data: points,
            len: write_idx
        }
    }

    const fn shift_data_for_piece<const N: usize>(piece: Piece) -> [FixedVec<Point, N>; 4] {
        [
            shift_frontier::<N>(BASE[piece as usize].1, Direction::Left),
            shift_frontier::<N>(BASE[piece as usize].1, Direction::Right),
            shift_frontier::<N>(BASE[piece as usize].1, Direction::Up),
            shift_frontier::<N>(BASE[piece as usize].1, Direction::Down),
        ]
    }
}

use pieces::Piece;

struct TetrisGame {
    grid: Grid<Mat>,
    current_height: i64,
    jets: Box<dyn Iterator<Item = Direction>>,
    next_piece: Box<dyn Iterator<Item = &'static Piece>>,
}

fn render_points(grid: &mut Grid<Mat>, material: Mat, points: &[Point], at: &Vector) {
    for point in points {
        grid.set(&(point + at), material);
    }
}

const FIELD_WIDTH: i64 = 7;

fn read_game() -> TetrisGame {
    let mut jets: Vec<Direction> = Vec::new();
    for char in input_all().chars() {
        match char {
            '<' => jets.push(Direction::Left),
            '>' => jets.push(Direction::Right),
            ch if ch.is_whitespace() => (),
            ch => panic!("unexpected input: '{}'", ch),
        }
    }

    TetrisGame{
        grid: Grid::new(FIELD_WIDTH, 2022 * 4, Mat::Air),
        current_height: 0,
        jets: Box::new(jets.into_iter().cycle()),
        next_piece: Box::new(Piece::ALL.iter().cycle()),
    }
}

fn show_pieces(with_directions: bool) {
    fn display_points(points: &[Point]) {
        let mut grid: Grid<Mat> = Grid::new(4, 4, Mat::Air);
        render_points(&mut grid, Mat::Rock, points, &Vector{ xd: 0, yd: 0 });
        grid.dump_part(&Point{ x: 0, y: 3 }, &Vector{ xd: 4, yd: -4 });
    }

    for piece in Piece::ALL {
        println!("{:?}", piece);
        display_points(pieces::points(piece));

        if ! with_directions {
            continue
        }
        for direction in Direction::ALL {
            println!("{:?} -> {:?}", piece, direction);
            let points: Vec<Point> = pieces::shifted_points(piece, direction).iter()
                .map(|p| p - &direction.as_vector()).collect();
            display_points(&points[..]);
        }
    }
}

fn push_piece(grid: &mut Grid<Mat>, piece: Piece, current_position: Point, direction: Direction) -> bool {
    let pos_vector = current_position.as_vector();
    for new_point in pieces::shifted_points(piece, direction) {
        let new_point = &(new_point + &pos_vector);
        match grid.get_or_default(new_point, Mat::Rock) {
            Mat::Rock => return false,
            Mat::Air => (),
        }
    }
    for old_point in pieces::shifted_points(piece, direction.opposite()) {
        let old_point = &(old_point + &pos_vector);
        let old_point = &(old_point + &direction.as_vector());
        grid.set(old_point, Mat::Air);
    }
    for new_point in pieces::shifted_points(piece, direction) {
        let new_point = &(new_point + &pos_vector);
        grid.set(new_point, Mat::Rock);
    }
    true
}

fn simulate_piece(game: &mut TetrisGame) {
    let piece = game.next_piece.next().unwrap();

    let mut position = Point{
        x: 2,
        y: game.current_height + 3,
    };

    render_points(&mut game.grid, Mat::Rock, pieces::points(*piece), &position.as_vector());

    loop {
        let jet = game.jets.next().unwrap();

        if push_piece(&mut game.grid, *piece, position, jet) {
            position = &position + &jet.as_vector();
        }

        if push_piece(&mut game.grid, *piece, position, Direction::Down) {
            position = &position + &Direction::Down.as_vector();
        } else {
            break;
        }
    }

    game.current_height = std::cmp::max(game.current_height, position.y + pieces::dims(*piece).h);
}

pub fn day_main() {
    // show_pieces(false);

    // let mut grid = Grid::<Mat>::new(7, 10, Mat::Air);

    // let current_piece = Piece::Cross;
    // let mut current_pos = Point{ x: 2, y: 0};
    // let mut step = 0;
    // render_points(&mut grid, Mat::Rock, pieces::points(current_piece), &current_pos.as_vector());

    // println!("");
    // println!("Step {}", step);
    // grid.dump_part_default(&Point{ x: -1, y: 9 }, &Vector{ xd: 9, yd: -11 }, Mat::Rock);


    // for direction in [Direction::Left, Direction::Left, Direction::Left] {

    //     if push_piece(&mut grid, current_piece, current_pos.clone(), direction) {
    //         current_pos = &current_pos + &direction.as_vector();
    //     }

    //     step += 1;
    //     println!("");
    //     println!("Step {}", step);
    //     grid.dump_part_default(&Point{ x: -1, y: 9 }, &Vector{ xd: 9, yd: -11 }, Mat::Rock);
    // }

    let mut game = read_game();

    // const ROUNDS: usize = 1_000_000_000_000;
    const ROUNDS: usize = 2022;
    // const ROUNDS: usize = 10;

    for _ in 0..ROUNDS {
        simulate_piece(&mut game);
        // println!("");
        // let display_height = game.current_height + 3;
        // game.grid.dump_part_default(&Point{ x: -1, y: display_height }, &Vector{ xd: 9, yd: -display_height - 2}, Mat::Rock);
    }
    println!("Tower height after {} rounds: {}", ROUNDS, game.current_height);

}