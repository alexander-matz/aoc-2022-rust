pub struct Grid<T: Copy> {
    data: Vec<T>,
    width: i32,
    height: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct Direction {
    pub xd: i32,
    pub yd: i32,
}

impl Direction {
    pub fn l1_norm(&self) -> i32 {
        self.xd.abs() + self.yd.abs()
    }

    pub fn signum(&self) -> Direction {
        fn aux(x: i32) -> i32 {
            if x < 0 {
                -1
            } else if x == 0 {
                0
            } else {
                1
            }
        }
        Direction{
            xd: aux(self.xd),
            yd: aux(self.yd),
        }
    }
}

impl std::ops::Add<&Direction> for &Point {
    type Output = Point;

    fn add(self, rhs: &Direction) -> Self::Output {
        Point{
            x: self.x + rhs.xd,
            y: self.y + rhs.yd,
        }
    }
}

impl std::ops::Sub<&Direction> for &Point {
    type Output = Point;

    fn sub(self, rhs: &Direction) -> Self::Output {
        Point{
            x: self.x - rhs.xd,
            y: self.y - rhs.yd,
        }
    }
}

impl std::ops::Sub<&Point> for &Point {
    type Output = Direction;

    fn sub(self, rhs: &Point) -> Self::Output {
        Direction{
            xd: self.x - rhs.x,
            yd: self.y - rhs.y,
        }
    }
}

#[allow(dead_code)]
impl <T: Copy> Grid<T> {

    pub fn new(width: i32, height: i32, value: T) -> Grid<T> {
        Grid{
            data: vec![value; (width * height) as usize],
            width: width,
            height: height,
        }
    }

    pub fn get(&self, p: &Point) -> T {
        assert!(p.x >= 0 && p.x < self.width);
        assert!(p.y >= 0 && p.y < self.height);
        self.data[(p.x + p.y * self.width) as usize]
    }

    pub fn set(&mut self, p: &Point, value: T) {
        assert!(p.x >= 0 && p.x < self.width);
        assert!(p.y >= 0 && p.y < self.height);
        self.data[(p.x + p.y * self.width) as usize] = value
    }

    pub fn is_in_bounds(&self, p: &Point) -> bool {
        p.x >= 0 && p.x < self.width && p.y >= 0 && p.y < self.height
    }
}

#[allow(dead_code)]
impl <T: Copy + std::fmt::Display> Grid<T> {
    pub fn dump(&self) {
        assert!(self.width <= 30 && self.height <= 30);
        for row in 0..self.height {
            for col in 0..self.width {
                print!("{}", self.get(&Point{ x: col, y: row }))
            }
        }
        println!("")
    }

    pub fn dump_part(&self, from: &Point, size: &Direction) {
        assert!(size.xd.abs() <= 30 && size.yd.abs() <= 30);
        assert!(self.is_in_bounds(from));
        let to = from + size;
        assert!(self.is_in_bounds(&(&to - &size.signum())));
        for row in from.y .. to.y {
            for col in from.x .. to.x {
                print!("{}", self.get(&Point{ x: col, y: row }))
            }
            println!("")
        }
    }
}