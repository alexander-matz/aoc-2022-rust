pub mod aoc {

    use std::collections::HashMap;

    #[derive(Debug)]
    enum Entry {
        File(usize),
        Dir(HashMap<String, Entry>),
    }

    #[derive(Debug)]
    struct Crawler {
        fs: Entry,
        pwd: Vec<String>
    }

    fn make_dir() -> Entry {
        Entry::Dir(HashMap::new())
    }

    fn make_file(size: usize) -> Entry {
        Entry::File(size)
    }

    impl Crawler {
        fn new() -> Crawler {
            Crawler{
                fs: make_dir(),
                pwd: Vec::new(),
            }
        }

        fn cd(&mut self, dir: &str) {
            match dir {
                "/" => self.pwd.clear(),
                ".." => { self.pwd.pop(); },
                other => self.pwd.push(other.to_owned()),
            }
        }

        fn add(&mut self, name: &str, entry: Entry) {
            let mut cursor = &mut self.fs;
            for part in self.pwd.iter() {
                match cursor {
                    Entry::File(_) => panic!("Encountered file in path {:?}", self.pwd),
                    Entry::Dir(map) => {
                        match map.get_mut(part) {
                            Some(entry) => { cursor = entry },
                            None => panic!("Did not find part {} of path {:?}", part, self.pwd)
                        }
                    }
                }
            }
            match cursor {
                Entry ::File(_) => panic!("File at end of path {:?}", self.pwd),
                Entry::Dir(ref mut map) => map.insert(name.to_owned(), entry)
            };
        }


    }

    #[allow(dead_code)]
    fn dump(fs: &Entry) {
        fn helper(current: &Entry, name: &str, prefix: &str) -> () {
            match current {
                Entry::File(size) => println!("{}{} ({})", prefix, name, size),
                Entry::Dir(items) => {
                    println!("{}{}:", prefix, name);
                    let mut new_prefix = prefix.to_owned();
                    new_prefix.push_str("  ");
                    for (name, item) in items.iter() {
                        helper(item, name, &new_prefix);
                    }
                }
            }
        }
        helper(fs, "/", "");
    }

    #[derive(Debug)]
    enum Type {
        File,
        Dir,
    }

    fn traverse_with_sizes<EntryFn, Ret>(entry: &Entry, entry_fn: &EntryFn) -> (usize, Vec<Ret>)
    where EntryFn: Fn(usize, Type) -> Ret
    {
        match entry {
            Entry::File(size) => {
                (*size, Vec::from([entry_fn(*size, Type::File)]))
            },
            Entry::Dir(map) => {
                let mut size_acc = 0;
                let mut ret_acc: Vec<Ret> = Vec::new();
                for (_, child_entry) in map.iter() {
                    let (size, mut ret) = traverse_with_sizes(child_entry, entry_fn);
                    size_acc += size;
                    ret_acc.append(&mut ret);
                }
                ret_acc.push(entry_fn(size_acc, Type::Dir));
                (size_acc, ret_acc)
            }
        }
    }

    #[allow(dead_code)]
    pub fn day_main_part1() {
        let on_line = |line: &str, mut crawler: Crawler| -> Crawler {
            if let Some(dir) = line.strip_prefix("$ cd ") {
                crawler.cd(dir);
                return crawler;
            }
            if let Some(name) = line.strip_prefix("dir ") {
                crawler.add(name, make_dir());
                return crawler;
            }
            if line.starts_with("$ ls") {
                return crawler;
            }
            if let Some((size_str, name)) = line.split_once(' ') {
                let size = size_str.parse::<usize>().unwrap();
                crawler.add(name, make_file(size));
            }
            crawler
        };

        let on_done = std::convert::identity;

        let crawler = crate::util::aoc::run_on_input(Crawler::new(), on_line, on_done);

        let entry_fn = |size, entry_type| -> Option<usize> {
            match entry_type {
                Type::Dir => Some(size),
                _ => None,
            }
        };
        let (root_size, directory_sizes) = traverse_with_sizes(&crawler.fs, &entry_fn);
        let small_directories = directory_sizes.iter()
            .map(|maybe_value| maybe_value.unwrap_or(0))
            .filter(|size| *size <= 100_000).fold(0, |acc, x| acc + x);
        println!("Sum of directories with size < 100_000: {}", small_directories);

        const TOTAL_DISK_SPACE: usize = 70_000_000;
        const REQUIRED_DISK_SPACE: usize = 30_000_000;
        let free = TOTAL_DISK_SPACE - root_size;
        assert!(free < REQUIRED_DISK_SPACE);
        let missing = REQUIRED_DISK_SPACE - free;

        let by_size = {
            let mut copy = directory_sizes.clone();
            copy.sort();
            copy
        };
        let smallest_required = by_size.iter().find(|size| size.unwrap_or(0) >= missing).unwrap().unwrap();
        println!("size of /: {}, free: {}, missing: {}", root_size, free, missing);
        println!("Smallest directory sufficient to free up {} has size {}", REQUIRED_DISK_SPACE, smallest_required);
    }

    #[allow(non_upper_case_globals, dead_code)]
    pub const day_main: fn() = day_main_part1;
}