use std::{fs, vec};

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Couldnt read Input File");

    let reindeers = parse_input(&input);
    println!("Part 1: {}", part_one(&reindeers));
    println!("Part 2: {}", part_two(&reindeers, 2503));
}

#[derive(Clone)]
struct Reindeer {
    name: String,
    top_speed: u16,
    top_speed_duration: u16,
    rest_duration: u16,
}

impl Reindeer {
    fn fly_for(&self, s: u32) -> u32 {
        let period_time = (self.top_speed_duration + self.rest_duration) as u32;
        let period_distance = (self.top_speed * self.top_speed_duration) as u32;
        // println!("{}", period_distance);
        let periods_in_s = s / period_time;
        // println!("{}", periods_in_s);
        let mut distance = periods_in_s * period_distance;
        let left_seconds = s % period_time;

        if left_seconds < self.top_speed_duration as u32 {
            distance += left_seconds * self.top_speed as u32;
        } else {
            distance += period_distance;
        }
        distance
    }
}

fn parse_input(input: &String) -> Vec<Reindeer> {
    let mut reindeers = Vec::new();
    for line in input.lines() {
        let line = line.split(" ").collect::<Vec<_>>();
        let reindeer = Reindeer {
            name: line[0].to_string(),
            top_speed: line[3].parse::<u16>().unwrap(),
            top_speed_duration: line[6].parse::<u16>().unwrap(),
            rest_duration: line[13].parse::<u16>().unwrap(),
        };
        reindeers.push(reindeer);
    }
    reindeers
}

fn part_one(reindeers: &Vec<Reindeer>) -> u32 {
    let mut max_disance = 0;
    let mut max_reindeer = &reindeers[0];
    for reindeer in reindeers {
        let distance = reindeer.fly_for(2503);
        if distance > max_disance {
            max_disance = distance;
            max_reindeer = reindeer;
        }
    }
    println!("{}", max_reindeer.name);
    max_disance
}

fn part_two(reindeers: &Vec<Reindeer>, s: u32) -> u32 {
    let mut points = vec![0 as u32; reindeers.len()];
    for s in 1..s + 1 {
        let mut max_distance = 0 as u32;
        let mut max_distance_ids = Vec::new();
        for (i, reindeer) in reindeers.into_iter().enumerate() {
            let distance = reindeer.fly_for(s);

            if distance > max_distance {
                max_distance_ids.clear();
                max_distance_ids.push(i);
                max_distance = distance;
            } else if distance == max_distance {
                max_distance_ids.push(i);
            }
        }
        for id in max_distance_ids {
            points[id] += 1;
        }
    }

    let mut max_points = 0;
    let mut max_reindeer_id = 0;
    for (i, p) in points.into_iter().enumerate() {
        if p > max_points {
            //only works for one winner
            max_points = p;
            max_reindeer_id = i;
        }
    }
    println!("{}", reindeers[max_reindeer_id].name);
    max_points
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_reindeers() -> Vec<Reindeer> {
        let mut reeindeers = Vec::new();
        let comet = Reindeer {
            name: "Comet".to_string(),
            top_speed: 14,
            top_speed_duration: 10,
            rest_duration: 127,
        };
        let dancer = Reindeer {
            name: "Dancer".to_string(),
            top_speed: 16,
            top_speed_duration: 11,
            rest_duration: 162,
        };
        reeindeers.push(comet);
        reeindeers.push(dancer);
        reeindeers
    }

    #[test]
    fn test_fly_for() {
        let reindeers = setup_reindeers();
        let fly_duration = 1000;
        assert_eq!(1120, reindeers[0].fly_for(fly_duration));
        assert_eq!(1056, reindeers[1].fly_for(fly_duration));
    }

    #[test]
    fn test_part_two() {
        let reindeers = setup_reindeers();
        assert_eq!(689, part_two(&reindeers, 1000));
    }
}
