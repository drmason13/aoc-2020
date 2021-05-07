use aoc_runner_derive::{aoc, aoc_generator};
use std::{
    collections::{HashMap, HashSet},
    ops::Range,
};

type Condition = (Range<u32>, Range<u32>);
type Fields = HashMap<String, Condition>;

#[derive(Debug, PartialEq)]
pub struct Notes {
    fields: Fields,
    your_ticket: Vec<u32>,
    nearby_tickets: Vec<Vec<u32>>,
}

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Notes {
    let lines = input.lines();
    let mut parsing_step = 0;

    let mut fields: Fields = HashMap::new();
    let mut your_ticket: Vec<u32> = Vec::new();

    let mut nearby_tickets: Vec<Vec<u32>> = Vec::new();

    for line in lines {
        match line {
            "your ticket:" | "nearby tickets:" => {
                parsing_step += 1;
                continue;
            }
            "" => continue,
            _ => {}
        }
        match parsing_step {
            0 => {
                // parse condition
                let mut parts = line.split(": ");
                let key = parts.next().unwrap();
                let mut ranges = parts.next().unwrap().split(" or ").map(|r| {
                    let mut ends = r.split('-');
                    std::ops::Range {
                        start: ends.next().unwrap().parse().unwrap(),
                        end: ends.next().unwrap().parse::<u32>().unwrap() + 1,
                    }
                });

                fields.insert(key.into(), (ranges.next().unwrap(), ranges.next().unwrap()));
            }
            1 => {
                // parse your_ticket
                your_ticket = line.split(',').map(|n| n.parse().unwrap()).collect();
            }
            2 => {
                // parse a nearby_ticket
                nearby_tickets.push(line.split(',').map(|n| n.parse().unwrap()).collect());
            }
            _ => unreachable!(),
        }
    }

    Notes {
        fields,
        your_ticket,
        nearby_tickets,
    }
}

#[aoc(day16, part1)]
pub fn part1(input: &Notes) -> u32 {
    let mut ticket_scanning_error_rate = 0;

    for ticket in input.nearby_tickets.iter() {
        for value in ticket {
            if input
                .fields
                .values() // conditions
                .all(|(range1, range2)| !range1.contains(value) && !range2.contains(value))
            {
                ticket_scanning_error_rate += value;
            }
        }
    }
    ticket_scanning_error_rate
}

#[aoc(day16, part2)]
pub fn part2(input: &Notes) -> u32 {
    let valid_tickets = input
        .nearby_tickets
        .iter()
        .filter(|ticket| {
            ticket.iter().all(|value| {
                input
                    .fields
                    .values() // conditions
                    .any(|(range1, range2)| range1.contains(value) || range2.contains(value))
            })
        })
        .cloned()
        .collect::<Vec<Vec<u32>>>();

    // track a set of all unassigned fields, once a field is assigned it isn't tested against anymore
    let mut unassigned_fields = HashSet::new();
    for field in input.fields.keys() {
        unassigned_fields.insert(field);
    }

    let mut field_assignments: HashMap<String, usize> = HashMap::new();

    // loop through each position in the ticket values
    for i in 0..input.fields.keys().len() {
        // track which fields this position (i) cannot be
        let mut ruled_out_fields = HashSet::new();
        for current_ticket in &valid_tickets {
            // rule out fields for this position using the current ticket
            let value = current_ticket[i];
            let ruled_out_fields_clone = ruled_out_fields.clone();
            let contender_fields = unassigned_fields.difference(&ruled_out_fields_clone);

            for &field_name in contender_fields {
                let (range1, range2) = input.fields.get(field_name).unwrap();
                if !(range1.contains(&value) || range2.contains(&value)) {
                    // rule out this field
                    ruled_out_fields.insert(field_name);
                }
            }

            if unassigned_fields.len() - ruled_out_fields.len() == 1 {
                // done
                let &field_name = unassigned_fields
                    .difference(&ruled_out_fields)
                    .next()
                    .unwrap();
                // record the field and the position
                field_assignments.insert(field_name.clone(), i);
                dbg!(field_name);
                break;
            }
        }
    }

    assert_eq!(field_assignments.keys().len(), input.fields.keys().len());

    0
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &'static str = indoc! {"
        class: 1-3 or 5-7
        row: 6-11 or 33-44
        seat: 13-40 or 45-50

        your ticket:
        7,1,14

        nearby tickets:
        7,3,47
        40,4,50
        55,2,20
        38,6,12"};

    const TEST_INPUT_PART2: &'static str = indoc! {"
        class: 0-1 or 4-19
        row: 0-5 or 8-19
        seat: 0-13 or 16-19
        
        your ticket:
        11,12,13
        
        nearby tickets:
        3,9,18
        15,1,5
        5,14,9"};

    #[test]
    fn notes_parsing_works() {
        let mut fields = HashMap::new();
        fields.insert(
            "class".into(),
            (Range { start: 1, end: 4 }, Range { start: 5, end: 8 }),
        );
        fields.insert(
            "row".into(),
            (Range { start: 6, end: 12 }, Range { start: 33, end: 45 }),
        );
        fields.insert(
            "seat".into(),
            (Range { start: 13, end: 41 }, Range { start: 45, end: 51 }),
        );
        let expected = Notes {
            fields,
            your_ticket: vec![7, 1, 14],
            nearby_tickets: vec![
                vec![7, 3, 47],
                vec![40, 4, 50],
                vec![55, 2, 20],
                vec![38, 6, 12],
            ],
        };

        assert_eq!(expected, input_generator(TEST_INPUT));
    }

    #[test]
    fn part1_works() {
        assert_eq!(71, part1(&input_generator(TEST_INPUT)));
    }

    #[test]
    fn part2_works() {
        assert_eq!(71, part2(&input_generator(TEST_INPUT_PART2)));
    }
}
