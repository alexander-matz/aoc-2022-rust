pub mod aoc {
    use crate::util::aoc::input_lines_indices;
    use std::collections::HashSet;

    type TreeType = i32;

    struct SquareBuf {
        buf: Vec<TreeType>,
        size: usize,
    }

    struct SquareBufIterator<'a> {
        buf: &'a SquareBuf,
        x: i32,
        y: i32,
        x_step: i8,
        y_step: i8,
    }

    impl <'a> Iterator for SquareBufIterator<'a> {
        type Item = (usize, usize, TreeType);

        fn next(&mut self) -> Option<Self::Item> {
            let size = self.buf.size as i32;
            if self.x < 0 || self.x >= size || self.y < 0 || self.y >= size {
                return None;
            }
            let result = Some((
                self.x as usize,
                self.y as usize,
                (self.buf.get(self.x as usize, self.y as usize))
            ));
            self.x += self.x_step as i32;
            self.y += self.y_step as i32;
            result
        }
    }

    impl SquareBuf {
        fn new(init: TreeType, size: usize) -> SquareBuf {
            SquareBuf { buf: vec![init; size * size], size: size }
        }

        fn get(&self, x: usize, y: usize) -> TreeType {
            assert!(x < self.size);
            assert!(y < self.size);
            self.buf[x + y * self.size]
        }

        fn set(&mut self, x: usize, y: usize, value: TreeType) {
            assert!(x < self.size);
            assert!(y < self.size);
            self.buf[x + y * self.size] = value;
        }

        fn row_iter<'a>(&'a self, y: usize, x_step: i8) -> SquareBufIterator<'a> {
            SquareBufIterator{
                buf: self,
                x: if x_step > 0 { 0 } else { (self.size-1) as i32 },
                y: y as i32,
                x_step: x_step,
                y_step: 0,
            }
        }

        fn col_iter<'a>(&'a self, x: usize, y_step: i8) -> SquareBufIterator<'a> {
            SquareBufIterator{
                buf: self,
                x: x as i32,
                y: if y_step > 0 { 0 } else { (self.size-1) as i32 },
                x_step: 0,
                y_step: y_step,
            }
        }

        #[allow(dead_code)]
        fn dump(&self) -> () {
            for row in 0..self.size {
                for col in 0..self.size {
                    print!("{}", self.get(col, row));
                }
                println!("");
            }
        }
    }

    fn read_trees() -> SquareBuf {
        let mut maybe_buf: Option<SquareBuf> = None;
        for (row, line) in input_lines_indices() {
            if maybe_buf.is_none() {
                maybe_buf = Some(SquareBuf::new(-1, line.len()));
            }
            let buf = maybe_buf.as_mut().unwrap();
            for (col, ch) in line.char_indices() {
                    assert!(ch >= '0' && ch <= '9');
                    let value = ch as i32 - '0' as i32;
                    buf.set(col, row, value);
            }
        }
        maybe_buf.unwrap()
    }

    struct State {
        highest: TreeType,
        visible: HashSet<(usize, usize)>,
    }
    fn count_increasing(mut state: State, value: (usize, usize, TreeType)) -> State {
        let (x, y, height) = value;
        if height > state.highest {
            state.visible.insert((x, y));
            State{
                highest: height,
                visible: state.visible,
            }
        } else {
            state
        }
    }

    fn scenic_score(forest: &SquareBuf, x: usize, y: usize) -> usize {
        let distance = |x_step: i32, y_step: i32| -> usize{
            let height = forest.get(x, y);
            let mut x = x as i32 + x_step;
            let mut y = y as i32 + y_step;
            let mut steps = 1;
            while x >= 0 && x < forest.size as i32 && y >= 0 && y < forest.size as i32 {
                steps += 1;
                if forest.get(x as usize, y as usize) >= height {
                    break;
                }
                x += x_step;
                y += y_step;

            }
            steps - 1
        };
        let left = distance(-1, 0);
        let right = distance(1, 0);
        let up = distance(0, -1);
        let down = distance(0, 1);
        left * right * up * down
    }

    #[allow(dead_code)]
    pub fn day_main_part() {
        let trees = read_trees();

        // trees.dump();

        let mut state = State{
            highest: -1,
            visible: HashSet::new(),
        };
        for idx in 0..trees.size {
            state.highest = -1;
            state = trees.row_iter(idx, 1).fold(state, count_increasing);
            state.highest = -1;
            state = trees.row_iter(idx, -1).fold(state, count_increasing);
            state.highest = -1;
            state = trees.col_iter(idx, 1).fold(state, count_increasing);
            state.highest = -1;
            state = trees.col_iter(idx, -1).fold(state, count_increasing);
        }

        println!("visible from outside: {}", state.visible.len());

        let mut max_score = 0;
        for row in 0..trees.size {
            for col in 0..trees.size {
                let score = scenic_score(&trees, col, row);
                if score > max_score {
                    max_score = score;
                }
            }
        }
        println!("highest scenic score: {}", max_score);
    }

    #[allow(non_upper_case_globals, dead_code)]
    pub const day_main: fn() = day_main_part;
}