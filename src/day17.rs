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
}

struct FixedVec<T, const N: usize> {
    data: [T; N],
    len: usize
}

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

    const fn as_points<const N: usize>(pairs: [(i32, i32); N]) -> [Point; N] {
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
        ( Dimensions{ w: 3, h: 3 }, &as_points([(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)]) ),
        ( Dimensions{ w: 3, h: 3 }, &as_points([(2, 0), (2, 1), (0, 2), (1, 2), (2, 2)]) ),
        ( Dimensions{ w: 1, h: 4 }, &as_points([(0, 0), (0, 1), (0, 2), (0, 3)]) ),
        ( Dimensions{ w: 2, h: 2 }, &as_points([(0, 0), (1, 0), (0, 1), (1, 1)]) ),
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
    current_height: i32,
    jets: Vec<Direction>,
    next_piece: Box<dyn Iterator<Item = &'static Piece>>,
}

fn render_points(grid: &mut Grid<Mat>, material: Mat, points: &[Point]) {
    for point in points {
        grid.set(point, material);
    }
}

const FIELD_WIDTH: i32 = 7;

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
        grid: Grid::new(FIELD_WIDTH, 10000, Mat::Air),
        current_height: 0,
        jets,
        next_piece: Box::new(Piece::ALL.iter().cycle()),
    }
}

fn show_pieces() {
    fn display_points(points: &[Point]) {
        let mut grid: Grid<Mat> = Grid::new(4, 4, Mat::Air);
        render_points(&mut grid, Mat::Rock, points);
        grid.dump_part(&Point{ x: 0, y: 3 }, &Vector{ xd: 4, yd: -4 });
    }

    for piece in Piece::ALL {
        println!("{:?}", piece);
        display_points(pieces::points(piece));

        for direction in Direction::ALL {
            println!("{:?} -> {:?}", piece, direction);
            let points: Vec<Point> = pieces::shifted_points(piece, direction).iter()
                .map(|p| p - &direction.as_vector()).collect();
            display_points(&points[..]);
        }
    }
}

// fn push_piece(grid: &mut Grid<Mat>, pieces: &Pieces, piece: Piece, current_position: Point, direction: Direction) -> bool {
//     todo!()
// }

pub fn day_main() {
    show_pieces();
    // read_game();
}