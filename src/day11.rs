#[allow(dead_code)]
pub mod aoc {
    use crate::parser::aoc::parser;
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

    #[allow(dead_code)]
    pub fn day_main_part() -> std::io::Result<()> {
        // let monkey_text = input_all();
        let parser = make_monkey_parser();
        println!("{:?}", parser.parse(MONKEY_TEXT, MONKEY_TEXT.chars()));
        Ok(())
    }

    #[allow(non_upper_case_globals, dead_code)]
    pub const day_main: fn() -> std::io::Result<()> = day_main_part;
}