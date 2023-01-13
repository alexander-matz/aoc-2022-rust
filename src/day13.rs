pub mod aoc {
    use std::str::Chars;
    use std::iter::Peekable;
    use std::cmp::Ordering;

    use crate::util::aoc::input_lines;

    #[derive(Debug, Clone, PartialEq)]
    enum Packet {
        One(i32),
        Many(Vec<Packet>),
    }

    impl std::fmt::Display for Packet {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Packet::One(val) => write!(f, "{}", val),
                Packet::Many(items) => {
                    write!(f, "[")?;
                    let mut first = true;
                    for item in items {
                        if ! first {
                            write!(f, ", ")?;
                        }
                        first = false;
                        write!(f, "{}", item)?;
                    }
                    write!(f, "]")
                }
            }
        }
    }

    fn read_packet(chars: &mut Peekable<Chars>) -> Packet {
        fn read_number(chars: &mut Peekable<Chars>) -> Packet {
            let mut acc = 0;
            loop {
                match chars.peek() {
                    Some(&ch) if ch >= '0' && ch <= '9' => {
                        acc = (acc * 10) + (ch as i32) - ('0' as i32);
                        chars.next();
                    },
                    _ => break
                }
            }
            Packet::One(acc)
        }

        fn read_list(chars: &mut Peekable<Chars>) -> Packet {
            let mut acc: Vec<Packet> = Vec::new();
            assert!(chars.next() == Some('['));
            while chars.peek() != Some(&']') {
                acc.push(read_packet(chars));
                if chars.peek() != Some(&',') {
                    break;
                }
                chars.next();
                while chars.peek() == Some(&' ') {
                    chars.next();
                }
            }
            chars.next();
            Packet::Many(acc)
        }

        match chars.peek() {
            Some(&ch) if ch == '[' => read_list(chars),
            Some(&ch) if ch >= '0' && ch <= '9' => read_number(chars),
            other => panic!("Expected '[' or '0'..'9', but got {:?}", other),
        }
    }

    fn compare_packets(left: &Packet, right: &Packet) -> Ordering {
        fn compare_lists(left: &Vec<Packet>, right: &Vec<Packet>) -> Ordering {
            let mut left_iter = left.iter();
            let mut right_iter = right.iter();
            loop {
                let lhs = left_iter.next();
                let rhs = right_iter.next();
                match lhs {
                    None => {
                        match rhs {
                            None => { return Ordering::Equal; },
                            Some(_) => { return Ordering::Less; }
                        }
                    },
                    Some(left_packet) => {
                        match rhs {
                            None => { return Ordering::Greater; }
                            Some(right_packet) => {
                                match compare_packets(left_packet, right_packet) {
                                    Ordering::Less => { return Ordering::Less; },
                                    Ordering::Greater => { return Ordering::Greater; },
                                    _ => ()
                                }
                            }
                        }
                    }
                }
            }
        }

        match left {
            Packet::One(left_number) => {
                match right {
                    Packet::One(right_number) => left_number.cmp(&right_number),
                    Packet::Many(right_list) =>
                        compare_lists(&vec![Packet::One(*left_number)], &right_list)
                }
            },
            Packet::Many(left_list) => {
                match right {
                    Packet::One(right_number) =>
                        compare_lists(left_list, &vec![Packet::One(*right_number)]),
                    Packet::Many(right_list) => compare_lists(left_list, right_list)
                }
            },
        }
    }

    pub fn day_main() {
        let packets: Vec<Packet> = input_lines()
            .filter(|line| !line.is_empty())
            .map(|line| read_packet(&mut line.chars().peekable()))
            .collect();

        {
            let mut index_sum = 0;
            let mut index = 1;
            let mut maybe_left = None as Option<Packet>;
            for packet in packets.iter() {
                match maybe_left {
                    None => maybe_left = Some(packet.clone()),
                    Some(left) => {
                        let order = compare_packets(&left,  &packet);
                        if order != Ordering::Greater {
                            index_sum += index;
                        }
                        maybe_left = None;
                        index += 1;
                    }
                }
            }
            println!("Index sum: {}", index_sum);
        }

        {
            let mut all_packets = packets.clone();
            let p2 = Packet::Many(vec![Packet::Many(vec![Packet::One( 2 )])]);
            let p6 = Packet::Many(vec![Packet::Many(vec![Packet::One( 6 )])]);
            all_packets.push(p2.clone());
            all_packets.push(p6.clone());

            all_packets.sort_by(compare_packets);

            let mut decoder_key = 1;
            for (index, packet) in all_packets.iter().enumerate() {
                if packet == &p2 || packet == &p6 {
                    println!("Multiplying decoder key {} with index {}", decoder_key, index + 1);
                    println!("  packet: {}", packet);
                    decoder_key *= (index + 1) as i32;
                }
            }
            println!("Decoder key: {}", decoder_key);
        }
    }
}