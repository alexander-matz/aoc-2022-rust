#[allow(dead_code)]
pub mod aoc {
    use crate::grid::Point;
    use crate::util::input_lines;

    #[derive(Debug)]
    struct Sensor {
        position: Point,
        beacon: Point,
    }

    type Range = std::ops::Range<i32>;

    fn clear_fields_on_row(sensor: &Sensor, row: i64) -> Option<Range> {
        let Sensor { position, beacon } = sensor;
        let clear_dist = (beacon - position).l1_norm();
        let y_diff = (position.y - row).abs();

        let width = clear_dist - y_diff;
        if width < 0 {
            None
        } else {
            let start = position.x - width;
            let end = position.x + width + 1;
            Some(std::ops::Range { start: start as i32, end: end as i32 })
        }
    }

    fn simplify_ranges(mut ranges: Vec<Range>) -> Vec<Range> {
        assert!(!ranges.is_empty());

        ranges.sort_by(|lhs, rhs| lhs.start.cmp(&rhs.start));

        let mut current_range = ranges.first().unwrap().clone();
        let mut read_ptr = 1 as usize;
        let mut write_ptr = 0 as usize;
        while read_ptr < ranges.len() {
            let range = ranges[read_ptr].clone();
            if range.start <= current_range.end {
                current_range.end = std::cmp::max(current_range.end, range.end);
            } else {
                ranges[write_ptr] = current_range;
                write_ptr += 1;
                current_range = range.clone()
            }
            read_ptr += 1;
        }
        ranges[write_ptr] = current_range;
        write_ptr += 1;
        ranges.truncate(write_ptr);
        ranges
    }

    fn clear_ranges_on_row(sensors: &Vec<Sensor>, row: i32) -> Vec<Range> {
        let ranges_on_row: Vec<Range> = sensors.iter()
                .map(|x| clear_fields_on_row(x, row as i64))
                .flatten().collect();
        simplify_ranges(ranges_on_row)
    }

    fn beacons_on_row(sensors: &Vec<Sensor>, row: i32) -> usize {
        use std::collections::BTreeSet;
        let mut beacons: BTreeSet<Point> = BTreeSet::new();
        beacons.extend(
            sensors.iter()
                .map(|sensor| sensor.beacon.clone())
                .filter(|beacon| beacon.y == row as i64)
        );
        beacons.len()
    }

    #[derive(Debug)]
    enum MaybeBeacons {
        None,
        One(i32),
        Multiple
    }
    fn find_beacon_options(sensors: &Vec<Sensor>, col_start: i32, col_end: i32, row: i32)
        -> MaybeBeacons
    {
        let mut maybe_beacons = MaybeBeacons::None;

        fn add_beacon_option(maybe_beacons: MaybeBeacons, col: i32) -> MaybeBeacons {
            match maybe_beacons {
                MaybeBeacons::None => MaybeBeacons::One(col),
                MaybeBeacons::One(other) if other == col => MaybeBeacons::One(col),
                _ => MaybeBeacons::Multiple,
            }
        }

        let clear_ranges = clear_ranges_on_row(sensors, row);
        if clear_ranges.is_empty() {
            return MaybeBeacons::Multiple
        }

        let mut current_col = col_start;
        for range in clear_ranges {
            if range.start >= col_end {
                break
            }
            match range.start - current_col {
                x if x < 1 => (),
                x if x == 1 => maybe_beacons = add_beacon_option(maybe_beacons, current_col),
                x if x > 1 => maybe_beacons = MaybeBeacons::Multiple,
                _ => unimplemented!(),
            }
            current_col = range.end
        }

        maybe_beacons
    }

    #[cfg(test)]
    mod test {
        use super::*;

        //  012345678901
        // 0......#.....
        // 1.....###....
        // 2....#####...
        // 3...###S###..
        // 4....B####...
        // 5.....###....
        // 6......#.....
        //
        // We use half-open interval, so [3, 4) contains only the element 3

        #[test]
        fn test_clear_fields_on_row() {
            let sensor = Sensor{
                position: Point{  x: 6, y: 3},
                beacon: Point{ x: 4, y: 4},
            };

            assert_matches!(clear_fields_on_row(&sensor, -2), None);
            assert_matches!(clear_fields_on_row(&sensor, -1), None);
            assert_matches!(clear_fields_on_row(&sensor, 0), Some(Range{ start: 6, end: 7}));
            assert_matches!(clear_fields_on_row(&sensor, 5), Some(Range{ start: 5, end: 8}));
            assert_matches!(clear_fields_on_row(&sensor, 6), Some(Range{ start: 6, end: 7}));
            assert_matches!(clear_fields_on_row(&sensor, 7), None);
        }

        #[test]
        fn test_simplify_ranges() {
            simplify_ranges(vec![
                -2..3, 2..15
            ]);
        }
    }

    fn read_sensors() -> Vec<Sensor> {
        use crate::parser::parse_wildcard;
        let mut sensors = Vec::new();
        let pattern = "Sensor at x=*, y=*: closest beacon is at x=*, y=*";

        for line in input_lines() {
            if line.is_empty() { continue; }

            let captures = parse_wildcard(pattern, '*', &line).unwrap();
            sensors.push(Sensor{
                position: Point {
                    x: captures[0].parse().unwrap(),
                    y: captures[1].parse().unwrap(),
                },
                beacon: Point {
                    x: captures[2].parse().unwrap(),
                    y: captures[3].parse().unwrap(),
                }
            });
        }
        sensors
    }

    pub fn day_main() {
        let sensors = read_sensors();
        println!("Read {} sensors", sensors.len());

        {
            const ROW: i32 = 10;
            // const ROW: i32 = 2000000;

            let clear_on_row = clear_ranges_on_row(&sensors, ROW);

            let cleared_fields: usize = clear_on_row.iter().map(|range| range.len()).sum();
            let beacons_on_row = beacons_on_row(&sensors, ROW);
            println!("cleared fields: {}", cleared_fields - beacons_on_row);
        }

        {
            // const MAX_EXTEND: i32 = 20;
            const MAX_EXTEND: i32 = 4_000_000;

            let mut beacon_option = MaybeBeacons::None;
            println!("checking rows for beacon options");
            for row in 0 .. MAX_EXTEND+1 {
                match find_beacon_options(&sensors, 0, MAX_EXTEND+1, row) {
                    MaybeBeacons::None => (),
                    MaybeBeacons::One(col) => {
                        match beacon_option {
                            MaybeBeacons::None => {
                                beacon_option = MaybeBeacons::One(col);
                                println!("Beacon option on x={},y={}, frequency={}",
                                    col, row, col as i64 * 4_000_000 + row as i64);
                            },
                            _ => panic!("Another beacon option on row {}", row),
                        }
                    },
                    MaybeBeacons::Multiple => {
                        panic!("Multiple beacon options on row {}", row);
                    }
                }
            }
        }
    }
}