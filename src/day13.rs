use aoc_runner_derive::{aoc_generator, aoc};

#[derive(Clone, Debug)]
pub struct Bus {
    id: u32,
    offset: u32,
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> (u32, Vec<Bus>) {
    let mut input = input.lines();
    let leaving_time = input.next().unwrap().trim().parse::<u32>().unwrap();
    let buses = input.next().unwrap().split(',')
        .enumerate()
        .filter_map(|(i, string)| {
            string.parse::<u32>().ok().and_then(|id| Some(Bus { id, offset: i as u32 }))
        })
        .collect();
    
    (leaving_time, buses)
}

#[aoc(day13, part1)]
/// What is the ID of the earliest bus you can take to the airport multiplied by the number of minutes you'll need to wait for that bus?
pub fn part1((leaving_time, buses): &(u32, Vec<Bus>)) -> Result<u32, &'static str> {
    // find the bus times that are on or after the leaving time
    let (bus_id, wait_time) = buses.iter()
        .cloned()
        .map(|bus| {
            let mut bus_time = bus.id;
            while bus_time < *leaving_time {
               bus_time += bus.id;
            }
            (bus.id, bus_time - leaving_time)
        })
        .min_by_key(|&(_, wait_time)| wait_time).unwrap();
    
    Ok(bus_id * wait_time)        
}

#[aoc(day13, part2)]
pub fn part2((_, _buses): &(u32, Vec<Bus>)) -> Result<usize, &'static str> {
    // find a timestamp where buses[0] and buses[1] are aligned as required (staggered by bus[1]'s offset)
    // skipping buses that are out of order
    /*
    for i in 0..buses.len() {
        let mut t = 1;
        
        while buses[i] 
    }
    */
    unimplemented!()
}

#[cfg(test)]
mod test {
    use super::*;

    const PART1_INPUT: &'static str = "\
        939
        7,13,x,x,59,x,31,19";

    #[test]
    fn part1_works() {
        assert_eq!(295, part1(&input_generator(PART1_INPUT)).unwrap());
    }

    #[test]
    #[ignore]
    fn part2_works() {
        assert_eq!(1068781, part2(&input_generator(PART1_INPUT)).unwrap());
    }
}
