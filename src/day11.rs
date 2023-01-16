#[allow(dead_code)]
pub mod aoc {
    use crate::parser;
    use crate::util::input_all;
    use std::rc::Rc;
    use std::collections::VecDeque;

    #[derive(Debug, Clone)]
    enum Operator {
        Add,
        Mul,
    }

    #[derive(Debug, Clone)]
    enum Operand {
        Old,
        Fixed(i32),
    }

    #[derive(Debug, Clone)]
    struct Operation {
        op1: Operand,
        op: Operator,
        op2: Operand,
    }

    #[derive(Debug, Clone)]
    struct Monkey {
        index: u32,
        items: VecDeque<i64>,
        operation: Operation,
        divisor: i32,
        target_true: u32,
        target_false: u32,
    }

    const MONKEY_TEXT: &str = "Monkey 0:
    Starting items: 79, 98
    Operation: new = old * 19
    Test: divisible by 23
      If true: throw to monkey 2
      If false: throw to monkey 3

  Monkey 1:
    Starting items: 54, 65, 75, 74
    Operation: new = old + 6
    Test: divisible by 19
      If true: throw to monkey 2
      If false: throw to monkey 0";

    fn make_monkey_parser() -> Rc<dyn parser::Parser> {
        use parser::*;

        let ws = make_many(make_ws());
        let number = make_many(make_range('0', '9'));

        let monkey_idx = make_seq(vec![
            make_lit("Monkey "), make_capture(number.clone()), make_char(':')
        ]);
        let monkey_items = make_seq(vec![
            make_lit("Starting items: "), make_list(make_capture(number.clone()), make_lit(", "))
        ]);

        let operator = make_alt(make_char('+'), make_char('*'));
        let monkey_op = make_seq(vec![
            make_lit("Operation: new = "),
            make_capture(make_word()),
            ws.clone(),
            make_capture(operator),
            ws.clone(),
            make_capture(make_word())
        ]);

        let monkey_test = make_seq(vec![
            make_lit("Test: divisible by "), make_capture(number.clone())
        ]);

        let monkey_true = make_seq(vec![
            make_lit("If true: throw to monkey "), make_capture(number.clone())
        ]);

        let monkey_false = make_seq(vec![
            make_lit("If false: throw to monkey "), make_capture(number.clone())
        ]);

        let monkey_all = make_seq(vec![
            monkey_idx, ws.clone(),
            monkey_items, ws.clone(),
            monkey_op, ws.clone(),
            monkey_test, ws.clone(),
            monkey_true, ws.clone(),
            monkey_false, make_any(make_ws()),
        ]);

        make_many(monkey_all)
    }

    fn construct_monkey(capture: crate::parser::Captured) -> Monkey {
        use crate::parser::Captured::*;

        fn as_one(capture: &parser::Captured) -> &str {
            match capture {
                One(val) => val,
                x => panic!("Expected one, got {:?}", x)
            }
        }

        fn parse_as_operand(capture: &parser::Captured) -> Operand {
            match capture {
                One(old) if old == "old" => Operand::Old,
                One(num) => Operand::Fixed(num.parse::<i32>().unwrap()),
                x => panic!("Expected One for operand, but got {:?}", x)
            }
        }

        fn parse_as_operator(capture: &parser::Captured) -> Operator {
            match capture {
                One(op) if op == "+" => Operator::Add,
                One(op) if op == "*" => Operator::Mul,
                _ => panic!("Expected One for operator, but got something else")
            }
        }

        fn parse_as_operation(capture: &parser::Captured) -> Operation {
            match capture {
                Many(x) if x.len() == 3 =>
                    Operation{
                        op1: parse_as_operand(&x[0]),
                        op: parse_as_operator(&x[1]),
                        op2: parse_as_operand(&x[2]),
                    },
                x => panic!("Expected Many(3 elems), but got {:?}", x)
            }
        }

        match capture {
            Many(fields) => {
                let index = as_one(&fields[0]).parse().unwrap();
                let items = match &fields[1] {
                    Many(x) => x.into_iter()
                        .map(|x| as_one(x).parse().unwrap()).collect(),
                    _ => panic!("Expected Many but got Something else"),
                };
                let operation = parse_as_operation(&fields[2]);
                let divisor = as_one(&fields[3]).parse().unwrap();
                let target_true = as_one(&fields[4]).parse().unwrap();
                let target_false = as_one(&fields[5]).parse().unwrap();
                Monkey{
                    index,
                    items,
                    operation,
                    divisor,
                    target_true,
                    target_false,
                }
            },
            _ => panic!("Expected Many for monkey, but got something else")
        }
    }

    fn parse_input(input: &str) -> Vec<Monkey> {
        let parser = make_monkey_parser();
        let (rest, captures) = parser.parse(input, input.chars()).unwrap();
        assert!(rest.as_str() == "");
        match captures {
            parser::Captured::Many(monkey_descriptions) =>
                monkey_descriptions.into_iter().map(construct_monkey).collect(),
            _ => panic!("Expected Many for monkey list, but got sommething else")
        }
    }

    fn apply_operation(worry_level: i64, op: &Operation) -> i64 {
        let lhs = match op.op1 {
            Operand::Old => worry_level,
            Operand::Fixed(val) => val as i64,
        };
        let rhs = match op.op2 {
            Operand::Old => worry_level,
            Operand::Fixed(val) => val as i64,
        };
        match op.op {
            Operator::Add => lhs + rhs,
            Operator::Mul => lhs * rhs,
        }
    }

    fn run_round(monkeys: &mut Vec<Monkey>, inspection_counts: &mut Vec<usize>, divide: bool, modulo: i64) {
        assert!(inspection_counts.len() == monkeys.len());

        for i in 0..monkeys.len() {
            let op = monkeys[i].operation.clone();
            let div = monkeys[i].divisor as i64;

            while let Some(mut worry_level) = monkeys[i].items.pop_front() {
                inspection_counts[i] += 1;
                worry_level = apply_operation(worry_level, &op);
                if divide {
                    worry_level = worry_level / 3;
                } else {
                    worry_level = worry_level % modulo;
                }
                let target = match worry_level % div == 0 {
                    true => monkeys[i].target_true,
                    false => monkeys[i].target_false ,
                } as usize;
                monkeys[target].items.push_back(worry_level);
            }
        }
    }

    fn show_situation(round: i32, monkeys: &Vec<Monkey>) {
        println!("Round {}", round);
        for i in 0..monkeys.len() {
            let items: Vec<String> = monkeys[i].items.iter().map(|x| x.to_string()).collect();
            println!("  Monkey {}: {}", i, items.join(", "));
        }
    }

    fn find_highest_counts<T: PartialOrd + Clone>(counts: &Vec<T>) -> ((usize, T), (usize, T)) {
        assert!(counts.len() >= 2);
        let mut indexed: Vec<(usize, T)> = counts.iter().enumerate()
            .map(|(idx, val)| (idx, val.clone())).collect();

        indexed.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap().reverse());

        let (idx0, val0) = &indexed[0];
        let (idx1, val1) = &indexed[1];
        ((*idx0, val0.clone()), (*idx1, val1.clone()))
    }

    fn find_modulo(monkeys: &Vec<Monkey>) -> i64 {
        monkeys.iter().fold(1, |acc, monkey| acc * (monkey.divisor as i64))
    }

    #[allow(dead_code)]
    fn day_main_part() {
        let monkey_text = &input_all();
        let monkeys = parse_input(&monkey_text);
        let modulo = find_modulo(&monkeys);

        // {
        //     let mut monkeys_1 = monkeys.clone();
        //     let mut inspection_counts: Vec<usize> = monkeys.iter().map(|_| 0 as usize).collect();

        //     for i in 0..20 {
        //         show_situation(i, &monkeys_1);
        //         run_round(&mut monkeys_1, &mut inspection_counts, true, modulo);
        //     }

        //     let highest_counts = find_highest_counts(&inspection_counts);
        //     println!("{:?}", highest_counts);
        //     println!("multiplied: {}", highest_counts.0.1 * highest_counts.1.1);
        // }

        {
            let mut monkeys_2 = monkeys.clone();
            let mut inspection_counts: Vec<usize> = monkeys.iter().map(|_| 0 as usize).collect();

            for _ in 0..10000 {
                // show_situation(i, &monkeys_2);
                run_round(&mut monkeys_2, &mut inspection_counts, false, modulo);
            }

            let highest_counts = find_highest_counts(&inspection_counts);
            println!("{:?}", highest_counts);
            println!("multiplied: {}", highest_counts.0.1 * highest_counts.1.1);
        }
    }

    #[allow(non_upper_case_globals, dead_code)]
    pub const day_main: fn() = day_main_part;
}