pub mod aoc {

    use std::io;

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