pub mod aoc {
    struct RingBuffer<T, const N: usize> {
        buffer: [T; N],
        pos: usize,
    }

    impl <T, const N: usize> RingBuffer<T, N> {
        fn new(init: T) -> RingBuffer<T, N> where T: Copy {
            RingBuffer{
                buffer: [init; N],
                pos: 0,
            }
        }

        fn push(&mut self, value: T) -> () where T: Copy {
            self.buffer[self.pos] = value;
            self.pos = (self.pos + 1) % N;
        }

        fn last(&self) -> T where T: Copy {
            self.buffer[self.pos]
        }
    }

    #[allow(dead_code)]
    pub fn day_main_part1() {
        let on_line = |line: &str, previous| -> Option<usize> {
            if line.is_empty() {
                return previous;
            }
            const BUF_SIZE: usize = 4;
            type Detector = RingBuffer<char, BUF_SIZE>;

            let mut buffer = Detector::new(' ');
            for (pos, ch) in line.char_indices() {
                buffer.push(ch);
                if pos < 3 {
                    continue
                }
                let [w, x, y, z] = buffer.buffer;
                if w != x && w != y && w != z && x != y && x != z && y != z {
                    println!("match: {}, {}, {}, {}", w, x, y, z);
                    return Some(pos + 1)
                }
            }
            None
        };

        let on_done = std::convert::identity;
        let result = crate::util::run_on_input(None, on_line, on_done);
        println!("End of first 4 unique characters: {:?}", result);
    }

    #[allow(dead_code)]
    pub fn day_main_part2() {
        const SEQ_LENGTH: usize = 14;

        let on_line = |line: &str, previous| -> Option<usize> {
            if line.is_empty() {
                return previous;
            }

            type Detector = RingBuffer<char, SEQ_LENGTH>;
            let mut buffer = Detector::new(' ');

            const SIZE: usize = ('z' as usize) - ('a' as usize) + 1;

            let mut occurences = [0 as usize; SIZE];
            let mut multiple_count = 0;

            for (pos, ch) in line.char_indices() {
                assert!(ch >= 'a' && ch <= 'z');
                match buffer.last() {
                    ' ' => (),
                    ch => {
                        let idx = ch as usize - 'a' as usize;
                        assert!(occurences[idx] > 0);
                        occurences[idx] -= 1;
                        if occurences[idx] == 1 {
                            assert!(multiple_count > 0);
                            multiple_count -= 1;
                        }
                    }
                }

                buffer.push(ch);
                let idx = ch as usize - 'a' as usize;
                occurences[idx] += 1;
                if occurences[idx] == 2 {
                    multiple_count += 1;
                }

                if multiple_count == 0 && pos >= SEQ_LENGTH - 1 {
                    println!("match @ {}, {:?}", pos, buffer.buffer);
                    return Some(pos+1);
                }
            }
            None
        };

        let on_done = std::convert::identity;
        let result = crate::util::run_on_input(None, on_line, on_done);
        println!("End of first {} unique characters: {:?}", SEQ_LENGTH, result);
    }

    #[allow(non_upper_case_globals, dead_code)]
    pub const day_main: fn() = day_main_part2;
}