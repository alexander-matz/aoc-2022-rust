pub mod aoc {

    use std::io;
    use std::io::Read;

    pub struct EasyLines {}

    impl Iterator for EasyLines {
        type Item = String;

        fn next(&mut self) -> Option<Self::Item> {
            let mut buf = String::new();
            match io::stdin().read_line(&mut buf) {
                Ok(0) => None,
                Ok(_) => Some(buf.trim().to_owned()),
                Err(error) => panic!("Error: {}", error)
            }
        }
    }

    #[allow(dead_code)]
    pub fn input_lines() -> EasyLines {
        EasyLines{ }
    }

    #[allow(dead_code)]
    pub fn input_all() -> String {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf).unwrap();
        buf
    }

    pub struct EasyLinesIndices {
        line: usize,
    }

    impl Iterator for EasyLinesIndices {
        type Item = (usize, String);

        fn next(&mut self) -> Option<Self::Item> {
            let mut buf = String::new();
            match io::stdin().read_line(&mut buf) {
                Ok(0) => None,
                Ok(_) => {
                    self.line += 1;
                    Some((self.line - 1, buf.trim().to_owned()))
                },
                Err(error) => panic!("Error: {}", error)
            }
        }
    }

    #[allow(dead_code)]
    pub fn input_lines_indices() -> EasyLinesIndices {
        EasyLinesIndices{ line: 0 }
    }


    pub fn run_on_input<LineFn, FinFn, State, Result>(init: State, on_line: LineFn, on_done: FinFn) -> Result
    where
        LineFn: Fn(&str, State) -> State,
        FinFn: FnOnce(State) -> Result
    {
        let mut state = init;
        loop {
            let mut buffer = String::new();
            match io::stdin().read_line(&mut buffer) {
                Ok(0) => {
                    return on_done(state)
                },
                Ok(_) => {
                    let line = buffer.trim();
                    state = on_line(line, state);
                },
                Err(error) => panic!("Error: {}", error)
            }
        }
    }
}