pub mod aoc {

use std::io;

#[derive(Debug)]
struct Counter {
    maxnew: [i32; 3],
    current: i32,
}

fn insert_calories(old_array: &[i32; 3], val: i32) -> [i32; 3] {
    let mut new_array = [0, 0, 0];
    let mut candidate = val;
    for i in 0..3 {
        if candidate > old_array[i] {
            new_array[i] = candidate;
            candidate = old_array[i];
        } else {
            new_array[i] = old_array[i];
        }
    }
    new_array
}

fn process_line(line: &str, counter: Counter) -> Counter {
    if line.is_empty() {

        Counter{
            maxnew: insert_calories(&counter.maxnew, counter.current),
            current: 0
        }
    } else {
        Counter{
            maxnew: counter.maxnew,
            current: counter.current + line.parse::<i32>().unwrap()
        }
    }
}

#[allow(dead_code)]
pub fn day_main() -> io::Result<()> {
    let mut counter = Counter{
        maxnew: [0, 0, 0],
        current: 0,
    };
    loop {
        let mut buffer = String::new();
        match io::stdin().read_line(&mut buffer) {
            Ok(0) => {
                if counter.current != 0 {
                    counter = process_line("", counter)
                }
                println!("Counter: {:?}", &counter);
                println!("Sum of maxes: {}", counter.maxnew.iter().fold(0, |acc, x| x + acc));
                return Ok(())
            },
            Ok(_) => {
                let line = buffer.trim();
                counter = process_line(line, counter);
            },
            Err(error) => return Err(error)
        }
    }
}

}