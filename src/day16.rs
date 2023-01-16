#[allow(dead_code)]
pub mod aoc {

    use std::collections::{BTreeMap, BTreeSet};

    use crate::parser::aoc::parser::parse_wildcard;
    use crate::util::aoc::input_lines_nonempty;

    #[derive(Debug, Clone, Copy)]
    struct Flow(i32);

    #[derive(Debug, Clone, Copy)]
    struct Time(i32);

    #[derive(Debug, Clone, Copy)]
    struct NodeIdx(u32);

    const MAX_TIME: Time = Time(30);

    struct Valve {
        name: String,
        flow: Flow,
        edges: Vec<NodeIdx>
    }

    struct ValveFmt<'a>(&'a Vec<Valve>);

    impl <'a> std::fmt::Debug for ValveFmt<'a> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            writeln!(f, "Graph:")?;
            for valve in self.0 {
                write!(f, "  Valve{{ name: {}, flow: {}, edges: ", valve.name, valve.flow.0)?;
                let mut first = true;
                for edge in valve.edges.iter() {
                    if ! first {
                        write!(f, ", ")?;
                    }
                    first = false;
                    write!(f, "{}", self.0[edge.0 as usize].name)?;
                }
                writeln!(f, "")?;
            }
            Ok(())
        }
    }

    type Valves = Vec<Valve>;

    fn read_valves() -> Valves {
        let mut lut: BTreeMap<String, u32> = BTreeMap::new();
        let mut valves = Valves::new();
        let mut edges: Vec<Vec<String>> = Vec::new();

        let pattern = "Valve * has flow rate=*; tunnel* lead* to valve* *";

        for line in input_lines_nonempty() {
            let captures = parse_wildcard(pattern, '*', &line).unwrap();
            let name = captures[0].to_owned();
            let flow = captures[1].parse::<i32>().unwrap();
            let edge_names: Vec<String> = captures[5].split(", ").map(|x| x.to_owned()).collect();
            let idx = valves.len();
            valves.push(Valve{
                name: name.clone(),
                flow: Flow(flow),
                edges: Vec::new()
            });
            edges.push(edge_names);

            lut.insert(name.clone(), idx as u32);

        }

        for (node_idx, edge_list) in edges.iter().enumerate() {
            let edges:  Vec<NodeIdx> = edge_list.iter()
                .map(|name| lut.get(name).unwrap())
                .map(|x| NodeIdx(*x))
                .collect();
            valves[node_idx].edges = edges;
        }

        valves
    }

    fn find_valve_by_name(valves: &Valves, name: &str) -> Option<NodeIdx> {
        for (idx, valve) in valves.iter().enumerate() {
            if valve.name == name {
                return Some(NodeIdx(idx as u32))
            }
        }
        None
    }

    fn find_best_path(valves: &Valves) -> (Vec<NodeIdx>, Flow) {
        fn update_candidate(old: Option<(Vec<NodeIdx>, Flow)>, new: (Vec<NodeIdx>, Flow)) -> (Vec<NodeIdx>, Flow) {
            let mut new_reversed = new.0.clone();
            new_reversed.reverse();
            match old {
                None => {
                    new
                },
                Some(value) => {
                    let mut old_reversed = value.0.clone();
                    old_reversed.reverse();
                    if new.1.0 > value.1.0 {
                        new
                    } else {
                        value
                    }
                }
            }
        }

        fn find_sub(
            valves: &Valves,
            node: NodeIdx,
            mut visited: BTreeSet<u32>,
            time: Time,
            flow_delta: Flow,
            flow_total: Flow) -> (Vec<NodeIdx>, Flow)
        {
            let valve = &valves[node.0 as usize];

            if time.0 >= MAX_TIME.0 {
                return (vec![node], flow_total);
            }

            let (valve_flow_delta, valve_flow_total, valve_time) = {
                if valve.flow.0 == 0 || time.0 >= MAX_TIME.0 - 2 {
                    (flow_delta, flow_total, time)
                } else {
                    (
                        Flow(flow_delta.0 + valve.flow.0),
                        Flow(flow_total.0 + flow_delta.0),
                        Time(time.0 + 1)
                    )
                }
            };

            let next_flow_total = Flow(valve_flow_total.0 + valve_flow_delta.0);
            let next_time = Time(valve_time.0 + 1);

            visited.insert(node.0);
            let mut best = None as Option<(Vec<NodeIdx>, Flow)>;
            for edge in valve.edges.iter() {
                if visited.contains(&edge.0) {
                    continue;
                }
                best = Some(update_candidate(
                    best,
                    find_sub(
                        valves,
                        *edge,
                        visited.clone(),
                        next_time,
                        valve_flow_delta,
                        next_flow_total,
                    )
                ));
            }
            match best {
                Some((mut path, flow)) => {
                    path.push(node);
                    (path, flow)
                },
                None => (
                    vec![node],
                    Flow(flow_total.0 + flow_delta.0 * (MAX_TIME.0 - time.0))
                )
            }
        }
        let aa = find_valve_by_name(valves, "AA").unwrap();

        let mut best = find_sub(
            valves,
            aa,
            BTreeSet::new(),
            Time(0),
            Flow(0),
            Flow(0)
        );
        best.0.reverse();

        best
    }

    pub fn day_main() {
        let valves = read_valves();
        println!("{:?}", ValveFmt(&valves));

        let (path_idx, flow) = find_best_path(&valves);
        let path: Vec<String> = path_idx.iter().map(|idx| valves[idx.0 as usize].name.clone()).collect();

        println!("flow: {}", flow.0);
        println!("path: {:?}", path);
        println!("DOES NOT WORK");
    }

}