#[allow(dead_code)]
pub mod aoc {
    use crate::parser::aoc::parser;
    use crate::util::aoc::input_all;
    use std::rc::Rc;

    #[derive(Debug)]
    enum Operator {
        Add,
        Mul,
    }

    #[derive(Debug)]
    enum Operand {
        Old,
        Fixed(i32),
    }

    #[derive(Debug)]
    struct Monkey {
        index: u32,
        items: Vec<u32>,
        operator: Operator,
        op1: Operand,
        op2: Operand,
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
            make_lit("Monkey "),
            make_capture(number.clone()),
            make_char(':')
        ]);
        let monkey_items = make_seq(vec![
            make_lit("Starting items: "),
            make_list(make_capture(number.clone()), make_lit(", "))
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
            make_lit("Test: divisible by "),
            make_capture(number.clone())
        ]);

        let monkey_true = make_seq(vec![
            make_lit("If true: throw to monkey "),
            make_capture(number.clone())
        ]);

        let monkey_false = make_seq(vec![
            make_lit("If false: throw to monkey "),
            make_capture(number.clone())
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

    fn construct_monkey(capture: parser::Captured) -> Monkey {
        use parser::Captured::*;

        fn parse_as_number(capture: &parser::Captured) -> i32 {
            match capture {
                One(val) => val.parse::<i32>().unwrap(),
                _ => panic!("Expected One for number, but got something else"),
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

        fn parse_as_operation(capture: &parser::Captured) -> (Operand, Operator, Operand) {
            match capture {
                Many(x) if x.len() == 3 =>
                    (parse_as_operand(&x[0]),parse_as_operator(&x[1]), parse_as_operand(&x[2])),
                x => panic!("Expected Many(3 elems), but got {:?}", x)
            }
        }

        match capture {
            Many(fields) => {
                let index = parse_as_number(&fields[0]) as u32;
                let items:Vec<u32> = match &fields[1] {
                    Many(x) => x.into_iter().map(parse_as_number).map(|x| x as u32).collect(),
                    _ => panic!("Expected Many but got Something else"),
                };
                let (op1, operator, op2) = parse_as_operation(&fields[2]);
                let divisor = parse_as_number(&fields[3]);
                let target_true = parse_as_number(&fields[4]) as u32;
                let target_false = parse_as_number(&fields[5]) as u32;
                Monkey{
                    index,
                    items,
                    operator,
                    op1,
                    op2,
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

    #[allow(dead_code)]
    pub fn day_main_part() -> std::io::Result<()> {
        let monkey_text = &input_all();
        let monkeys = parse_input(&monkey_text);
        println!("{:?}", monkeys);
        Ok(())
    }

    #[allow(non_upper_case_globals, dead_code)]
    pub const day_main: fn() -> std::io::Result<()> = day_main_part;
}