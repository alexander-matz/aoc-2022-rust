#![allow(dead_code)]

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
pub struct Vector {
    pub xd: i32,
    pub yd: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct Dimensions {
    pub w: i32,
    pub h: i32,
}

impl Vector {
    pub fn l1_norm(&self) -> i32 {
        self.xd.abs() + self.yd.abs()
    }

    pub fn signum(&self) -> Vector {
        fn aux(x: i32) -> i32 {
            if x < 0 {
                -1
            } else if x == 0 {
                0
            } else {
                1
            }
        }
        Vector{
            xd: aux(self.xd),
            yd: aux(self.yd),
        }
    }
}

impl std::ops::Add<&Vector> for &Point {
    type Output = Point;

    fn add(self, rhs: &Vector) -> Self::Output {
        Point{
            x: self.x + rhs.xd,
            y: self.y + rhs.yd,
        }
    }
}

impl std::ops::Sub<&Vector> for &Point {
    type Output = Point;

    fn sub(self, rhs: &Vector) -> Self::Output {
        Point{
            x: self.x - rhs.xd,
            y: self.y - rhs.yd,
        }
    }
}

impl std::ops::Sub<&Point> for &Point {
    type Output = Vector;

    fn sub(self, rhs: &Point) -> Self::Output {
        Vector{
            xd: self.x - rhs.x,
            yd: self.y - rhs.y,
        }
    }
}

impl <T: Copy> Grid<T> {

    pub fn new(width: i32, height: i32, value: T) -> Grid<T> {
        Grid{
            data: vec![value; (width * height) as usize],
            width: width,
            height: height,
        }
    }

    pub fn from_array<const SIZE: usize>(width: i32, height: i32, array: [T; SIZE]) -> Grid<T> {
        assert!(SIZE == (width * height) as usize);
        Grid{
            data: Vec::from(array),
            width: width,
            height: height
        }
    }

    pub fn get(&self, p: &Point) -> T {
        assert!(p.x >= 0 && p.x < self.width);
        assert!(p.y >= 0 && p.y < self.height);
        self.data[(p.x + p.y * self.width) as usize]
    }

    pub fn get_or_default(&self, p: &Point, default: T) -> T {
        if p.x < 0 || p.x >= self.width || p.y < 0 || p.y >= self.height {
            default
        } else {
            self.data[(p.x + p.y * self.width) as usize]
        }
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

impl <T: Copy + std::fmt::Display> Grid<T> {
    fn dump_helper<GetterFn: Fn(&Point) -> T>(&self, from: &Point, size: &Vector, getter: GetterFn) {
        assert!(size.xd.abs() <= 30 && size.yd.abs() <= 30);
        let to = from + size;
        let mut row = from.y;
        let step = size.signum();
        while row != to.y {
            let mut col = from.x;
            while col != to.x {
                print!("{}", getter(&Point{ x: col, y: row }));
                col += step.xd;
            }
            println!("");
            row += step.yd;
        }
    }

    pub fn dump(&self) {
        self.dump_helper(&Point{ x: 0, y: 0 }, &Vector{ xd: self.width, yd: self.height }, |point| self.get(point));
    }

    pub fn dump_part(&self, from: &Point, size: &Vector) {
        assert!(self.is_in_bounds(from));
        assert!(self.is_in_bounds(&(&(from + size) - &size.signum())));

        self.dump_helper(from, size, |point| self.get(point));
    }

    pub fn dump_part_default(&self, from: &Point, size: &Vector, default: T) {
        self.dump_helper(from, size, |point| self.get_or_default(point, default));
    }
}