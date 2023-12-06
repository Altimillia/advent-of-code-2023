use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use itertools::{Itertools, min};
use nom::bytes::complete::{tag, take_until};
use nom::IResult;
use crate::tools::{parse_numbers, parse_numbers_u64};

pub fn part_one(input: String) -> impl Display {
    map_seeds_to_location(input)
}

fn map_seeds_to_location(input: String) -> u64 {
    let mut split = input.split("\n\n");
    let seeds = Seeds::parse(split.nth(0).unwrap()).unwrap().1;
    let maps:Vec<Map> = split.map(|section| Map::new(section)).collect();

    let mut locations:Vec<u64> = Vec::new();

    seeds.seeds.iter().for_each(|seed_start| {
        let mut tracking_value = seed_start.clone();
        maps.iter().for_each(|map| {
            tracking_value = map.map_input(tracking_value);
        });
        locations.push(tracking_value);
    });

    locations.iter().for_each(|location| {
        // println!("{}", location);
    });

    *locations.iter().min().unwrap()
}


pub fn part_two(input: String) -> impl Display {
    map_seeds_to_location_with_seed_range(input)
}


fn map_seeds_to_location_with_seed_range(input: String) -> u64 {
    let mut split = input.split("\n\n");
    let seeds = Seeds::parse(split.nth(0).unwrap()).unwrap().1;
    let maps:Vec<Map> = split.map(|section| Map::new(section)).collect();

    let mut locations:Vec<(u64,u64)> = Vec::new();

    seeds.seed_ranges.iter().for_each(|seed_start| {
        println!("Starting seed range {}", seed_start.start);
        for seed_value in (seed_start.start..seed_start.start + seed_start.range).step_by(25) {
            let mut tracking_value = seed_value.clone();
            maps.iter().for_each(|map| {
                tracking_value = map.map_input(tracking_value);
            });
            locations.push((seed_start.start, tracking_value));
        }
    });


    let (seed, location) = *locations.iter().min_by_key(|(seed, location)| location).unwrap();
    println!("{} {}", seed, location);
    location
}


fn map_seeds_to_location_with_seed_range_v2_final(input: String) -> u64 {
    let mut split = input.split("\n\n");
    let seeds = Seeds::parse(split.nth(0).unwrap()).unwrap().1;
    let maps:Vec<Map> = split.map(|section| Map::new(section)).collect();
    let seed_ranges:Vec<Range> = seeds.seed_ranges.iter().map(|seed_range| Range::new(seed_range.start, seed_range.range)).collect();

    let mut lowest:u64 = u64::MAX;
    seed_ranges.iter().for_each(|range| {
        println!("RANGE {} {}", range.intervals[0].start, range.intervals[0].length);
        let seed_start = range.intervals[0].start;
        let mut end_range = range.clone();
        maps.iter().for_each(|map| {
            end_range = map.apply_range_mapping(end_range.clone());
        });

        let final_mapping = end_range.get_final_mapping_ranges();
        final_mapping.iter().for_each(|kvp| {
            println!("{} - {}", kvp.1, kvp.0 + seed_start);
            lowest = std::cmp::min(lowest, *kvp.0);
        });
        println!("{}", end_range.intervals.iter().count());
    });
    println!("{}", lowest);

    0
}

#[derive(Debug, PartialEq)]
pub enum EntityType {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location
}

impl FromStr for EntityType {
    type Err = ();
    fn from_str(input: &str) -> Result<EntityType, Self::Err> {
        match input {
            "seed"  => Ok(EntityType::Seed),
            "soil"  => Ok(EntityType::Soil),
            "fertilizer"  => Ok(EntityType::Fertilizer),
            "water" => Ok(EntityType::Water),
            "light" => Ok(EntityType::Light),
            "temperature" => Ok(EntityType::Temperature),
            "humidity" => Ok(EntityType::Humidity),
            "location" => Ok(EntityType::Location),
            _      => Err(()),
        }
    }
}


pub enum MapType {
    Mapping(EntityType, EntityType)
}


struct Map {
    map_entries: Vec<MapEntry>,
    source_entity: EntityType,
    destination_entity: EntityType
}

impl Map {
    fn new(input_block: &str) -> Self {
        let first_line = input_block.lines().nth(0).unwrap();
        let map_type = Map::get_map_type(first_line).unwrap().1;

        let map_entries:Vec<MapEntry> = input_block.lines().skip(1).map(|line| MapEntry::from_str(line).unwrap()).collect();


        Map { source_entity: map_type.0, destination_entity: map_type.1, map_entries }
    }

    fn get_map_type(input_line: &str) -> IResult<&str, (EntityType, EntityType)> {
        let (input_line, source_entity) = take_until("-")(input_line)?;
        let (input_line, tag) = tag("-to-")(input_line)?;
        let (input_line, destination_entity) = take_until(" ")(input_line)?;

        Ok((input_line, (EntityType::from_str(source_entity).unwrap(), EntityType::from_str(destination_entity).unwrap())))
    }

    fn map_input(&self, entity_id: u64) -> u64 {
        for map_entry in &self.map_entries {
            if(map_entry.contains_source(entity_id)){
                return map_entry.get_destination(entity_id);
            }
        }

        entity_id
    }

    fn apply_range_mapping(&self, range: Range) -> Range {
        let mut range = range.clone();
        &self.map_entries.iter().for_each(|map_entry| range = map_entry.cut_range(range.clone()));

        let mut intervals:Vec<Interval> = vec![];
        range.intervals.iter().for_each(|interval| {
            intervals.push(Interval { start: *&self.map_input(interval.start), length: interval.length});
        });
        range.intervals = intervals;

        range
    }
}
#[derive(Clone)]
struct Interval {
    start: u64,
    length: u64
}
#[derive(Clone)]
struct Range {
    intervals: Vec<Interval>
}

impl Range {
    fn new(start: u64, length: u64) -> Self{
        return Range { intervals: vec![Interval { start, length }]}
    }

    fn get_final_mapping_ranges(&self) -> HashMap<u64, u64> {
        let mut start_to_end:HashMap<u64,u64> = HashMap::new();
        let mut current_index = 0;
        self.intervals.iter().for_each(|interval| {
           start_to_end.insert(interval.start, current_index);
            current_index += interval.length;
        });

        start_to_end
    }
}


struct MapEntry {
    destination_start: u64,
    source_start: u64,
    range: u64
}

impl MapEntry {
    fn contains_source(&self, entity_id: u64) -> bool {
        return self.source_start <= entity_id && entity_id <= self.source_start + self.range;
    }
    fn get_destination(&self, entity_id: u64) -> u64 {
        let range_offset = entity_id - self.source_start;
        self.destination_start + range_offset
    }

    fn cut_range(&self, range: Range) -> Range {
        let mut updated_range = Range { intervals:vec![] };
        for interval in range.intervals {
            if(interval.start + interval.length <= self.source_start || interval.start >= self.source_start + self.range)
            {
                updated_range.intervals.push(interval.clone());
            }
            else {
                if(interval.start < self.source_start){
                    updated_range.intervals.push(Interval { start: interval.start, length: self.source_start - interval.start })
                }

                let overlap_start = max(interval.start, self.source_start);
                let overlap_end = std::cmp::min(interval.start + interval.length, self.source_start + self.range);
                updated_range.intervals.push(Interval { start: overlap_start, length: overlap_end - overlap_start });

                if(interval.start + interval.length > self.source_start + self.range) {
                    updated_range.intervals.push(Interval { start: self.source_start + self.range, length: (interval.start + interval.length) - (self.source_start + self.range)})
                }
            }
        }
        updated_range
    }
}
impl FromStr for MapEntry {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let mut split = s.split_whitespace();
        let dest = parse_numbers_u64(split.nth(0).unwrap()).unwrap().1;
        let source = parse_numbers_u64(split.nth(0).unwrap()).unwrap().1;
        let range = parse_numbers_u64(split.nth(0).unwrap()).unwrap().1;

        Ok(MapEntry { destination_start: dest, source_start: source, range })
    }
}

struct Seed {
    start: u64,
    range: u64
}


struct Seeds {
    seeds: Vec<u64>,
    seed_ranges: Vec<Seed>
}
impl Seeds {
    fn parse(input_line: &str) -> IResult<&str, Self> {
        let (input_line, _) = tag("seeds: ")(input_line)?;
        let seeds = input_line
            .split_whitespace()
            .map(|value| parse_numbers_u64(value).unwrap().1)
            .collect();

        let seed_ranges = input_line.split_whitespace().collect::<Vec<&str>>().chunks(2).map(|chunk| {
            let start = parse_numbers_u64(chunk[0]).unwrap().1;
            let range = parse_numbers_u64(chunk[1]).unwrap().1;
            return Seed { start, range }
        }).collect();

        Ok((input_line, Seeds { seeds, seed_ranges }))
    }
}


#[cfg(test)]
mod tests {
    use super::{map_seeds_to_location, map_seeds_to_location_with_seed_range};
    #[test]
    fn mapping_can_get_locations() {
        let input = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#.to_string();

        let result = map_seeds_to_location(input);

        assert_eq!(result, 35);
    }

    #[test]
    fn mapping_can_get_locations_part_2() {
        let input = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#.to_string();

        let result = map_seeds_to_location_with_seed_range(input);

        assert_eq!(result, 46);
    }

}
