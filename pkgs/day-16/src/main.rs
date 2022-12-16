use std::collections::{HashMap, HashSet};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::{self, complete::line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    IResult,
};

#[derive(Debug, Eq, PartialEq, Hash)]
struct Valve<'a> {
    name: &'a str,
    flow_rate: u64,
    tunnels: Vec<&'a str>,
}

fn main() {
    let valves = parse(include_str!("in").trim_end())
        .unwrap()
        .1
        .into_iter()
        .map(|v| (v.name, v))
        .collect::<HashMap<&str, Valve>>();

    dbg!(part1(&valves));
    dbg!(part2(&valves));
}

fn parse(input: &str) -> IResult<&str, Vec<Valve>> {
    all_consuming(separated_list1(line_ending, valve))(input)
}
fn valve_name(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_ascii_uppercase())(input)
}
fn valve(input: &str) -> IResult<&str, Valve> {
    let (input, _) = tag("Valve ")(input)?;
    let (input, name) = valve_name(input)?;
    let (input, _) = tag(" has flow rate=")(input)?;
    let (input, flow_rate) = character::complete::u64(input)?;
    let (input, _) = alt((
        tag("; tunnel leads to valve "),
        tag("; tunnels lead to valves "),
    ))(input)?;
    let (input, tunnels) = separated_list1(tag(", "), valve_name)(input)?;
    Ok((
        input,
        Valve {
            name,
            flow_rate,
            tunnels,
        },
    ))
}

fn part1(valves: &HashMap<&str, Valve>) -> Option<u64> {
    let mut cache = HashMap::new();
    let open_valves = HashSet::new();
    solve_part1(1, "AA", 0, 0, &open_valves, valves, &mut cache)
}

fn part2(valves: &HashMap<&str, Valve>) -> Option<u64> {
    let mut cache = HashMap::new();
    let open_valves = HashSet::new();
    solve_part2(1, "AA", "AA", 0, 0, &open_valves, valves, &mut cache)
}

fn solve_part1<'a>(
    current_time: u32,
    my_location: &'a str,
    current_flow_rate: u64,
    current_score: u64,
    open_valves: &HashSet<&str>,
    all_valves: &HashMap<&str, Valve<'a>>,
    best_scores: &mut HashMap<(u32, &'a str, u64), u64>,
) -> Option<u64> {
    if current_time > 30 {
        return Some(current_score);
    }

    let state_key = (current_time, my_location, current_flow_rate);
    if let Some(&best_score) = best_scores.get(&state_key) {
        if best_score >= current_score {
            return None;
        }
    }
    best_scores.insert(state_key, current_score);

    let current_valve = all_valves.get(my_location).unwrap();

    let best_result_open_current =
        if current_valve.flow_rate > 0 && !open_valves.contains(my_location) {
            let mut new_open_valves = open_valves.clone();
            new_open_valves.insert(my_location);

            let new_score = current_score + current_flow_rate;
            let new_flow_rate = current_flow_rate + current_valve.flow_rate;
            solve_part1(
                current_time + 1,
                my_location,
                new_flow_rate,
                new_score,
                &new_open_valves,
                all_valves,
                best_scores,
            )
        } else {
            None
        };

    let best_result_down_tunnels = current_valve
        .tunnels
        .iter()
        .filter_map(|next_valve_name| {
            solve_part1(
                current_time + 1,
                next_valve_name,
                current_flow_rate,
                current_score + current_flow_rate,
                open_valves,
                all_valves,
                best_scores,
            )
        })
        .max();

    best_result_down_tunnels.max(best_result_open_current)
}

#[allow(clippy::too_many_arguments)]
fn solve_part2<'a>(
    minute: u32,
    my_location: &'a str,
    elephant_location: &'a str,
    current_flow_rate: u64,
    current_score: u64,
    open_valves: &HashSet<&str>,
    all_valves: &HashMap<&str, Valve<'a>>,
    cache: &mut HashMap<(u32, &'a str, &'a str, u64), u64>,
) -> Option<u64> {
    if minute > 26 {
        return Some(current_score);
    }

    let cache_key = (minute, my_location, elephant_location, current_flow_rate);
    if let Some(&cached_value) = cache.get(&cache_key) {
        if cached_value >= current_score {
            return None;
        }
    }
    cache.insert(cache_key, current_score);

    let my_valve = all_valves.get(my_location).unwrap();
    let elephant_valve = all_valves.get(elephant_location).unwrap();

    let can_open_my_valve = my_valve.flow_rate > 0 && !open_valves.contains(my_location);
    let can_open_elephant_valve =
        elephant_valve.flow_rate > 0 && !open_valves.contains(elephant_location);

    let mut results = Vec::new();

    // I open, elephant moves
    if can_open_my_valve {
        let mut new_open_valves = open_valves.clone();
        new_open_valves.insert(my_location);

        for new_elephant_location in elephant_valve.tunnels.iter() {
            results.push(solve_part2(
                minute + 1,
                my_location,
                new_elephant_location,
                current_flow_rate + my_valve.flow_rate,
                current_score + current_flow_rate,
                &new_open_valves,
                all_valves,
                cache,
            ));
        }
    }

    // I move, elephant opens
    if can_open_elephant_valve {
        let mut new_open_valves = open_valves.clone();
        new_open_valves.insert(elephant_location);

        for new_my_location in my_valve.tunnels.iter() {
            results.push(solve_part2(
                minute + 1,
                new_my_location,
                elephant_location,
                current_flow_rate + elephant_valve.flow_rate,
                current_score + current_flow_rate,
                &new_open_valves,
                all_valves,
                cache,
            ));
        }
    }

    // I open, elephant opens
    if can_open_elephant_valve && can_open_my_valve && my_location != elephant_location {
        let mut new_open_valves = open_valves.clone();
        new_open_valves.insert(elephant_location);
        new_open_valves.insert(my_location);

        results.push(solve_part2(
            minute + 1,
            my_location,
            elephant_location,
            current_flow_rate + my_valve.flow_rate + elephant_valve.flow_rate,
            current_score + current_flow_rate,
            &new_open_valves,
            all_valves,
            cache,
        ));
    }

    // I move, elephant moves
    for new_elephant_location in elephant_valve.tunnels.iter() {
        for new_my_location in my_valve.tunnels.iter() {
            results.push(solve_part2(
                minute + 1,
                new_my_location,
                new_elephant_location,
                current_flow_rate,
                current_score + current_flow_rate,
                open_valves,
                all_valves,
                cache,
            ));
        }
    }

    results.into_iter().flatten().max()
}
