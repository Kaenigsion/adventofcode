use itertools::Itertools;
use std::fs;

fn main() {
    let path = "day-five/puzzle_input.txt";
    let content = fs::read_to_string(path).expect("couldn't open file");
    // let location1 = get_nearest_location1(content.clone());
    // println!("p1: {location1}");

    let location2 = get_nearest_location2(content.clone());
    println!("p2: {location2}");
}

// p1
fn get_nearest_location1(content: String) -> usize {
    let (_, seed_line) = content.lines().next().unwrap().split_once(':').unwrap();
    let seeds: Vec<usize> = seed_line
        .trim()
        .split(" ")
        .map(|x| x.parse().unwrap())
        .collect();

    let mut table = Vec::with_capacity(7);

    for lines in content.split("\n\n").skip(1) {
        let mut ranges = (vec![], vec![]); // source, destination

        for line in lines.trim().lines().skip(1) {
            for (destination, source, range) in line
                .split(" ")
                .map(|x| x.trim().parse::<usize>().unwrap())
                .tuples()
            {
                ranges.0.push((source, source + range));
                ranges.1.push((destination, destination + range));
            }
        }
        table.push(ranges);
    }
    let mut locations: Vec<usize> = vec![];

    for seed in seeds {
        let mut previous = seed;
        for index in 0..7 {
            let mut i: Option<(usize, usize)> = None;
            table[index].0.iter().enumerate().for_each(|(k, x)| {
                if let Some((j, _)) = (x.0..x.1).find_position(|x| x == &previous) {
                    i = Some((k, j));
                }
            });
            if let Some((k, j)) = i {
                let start_end = table[index].1[k];
                let value = (start_end.0..start_end.1).nth(j).unwrap();
                previous = value;
            }
        }
        locations.push(previous);
    }

    *locations.iter().min().unwrap()
}

// p2
fn get_nearest_location2(content: String) -> usize {
    let (_, seed_line) = content.lines().next().unwrap().split_once(':').unwrap();
    let mut seeds: Vec<usize> = vec![];
    seed_line
        .trim()
        .split(" ")
        .map(|x| x.parse().unwrap())
        .tuples()
        .for_each(|(start, length)| seeds.append(&mut Vec::from_iter(start..(start + length))));

    let mut table = Vec::with_capacity(7);

    for lines in content.split("\n\n").skip(1) {
        let mut ranges = (vec![], vec![]); // source, destination

        for line in lines.trim().lines().skip(1) {
            for (destination, source, range) in line
                .split(" ")
                .map(|x| x.trim().parse::<usize>().unwrap())
                .tuples()
            {
                ranges.0.push((source, source + range));
                ranges.1.push((destination, destination + range));
            }
        }
        table.push(ranges);
    }

    let mut locations: usize = usize::MAX;

    for seed in seeds {
        let mut previous = seed;
        for index in 0..7 {
            let mut i: Option<(usize, usize)> = None;
            table[index].0.iter().enumerate().for_each(|(k, x)| {
                if let Some((j, _)) = (x.0..x.1).find_position(|x| x == &previous) {
                    i = Some((k, j));
                }
            });
            if let Some((k, j)) = i {
                let start_end = table[index].1[k];
                let value = (start_end.0..start_end.1).nth(j).unwrap();
                previous = value;
            }
        }

        println!("{}", previous);

        if previous < locations {
            locations = previous;
        }
    }

    locations
}
