#![allow(dead_code)]
use std::{collections::HashMap, cmp};

use advent_of_code::{read_to_lines, precompute_path, Valve};
use priority_queue::PriorityQueue;
use regex::Regex;

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
struct AgendaDuo {
    pos_i: String,
    pos_e: String,
    goal_i: String,
    goal_e: String,
    time: i32,
    time_i: i32,
    time_e: i32,
    acc_flow_rate: i32,
    acc_relief: i32,
    opened: Vec<String>,
    visited_i: Vec<String>,
    visited_e: Vec<String>,
    remaining: Vec<String>,
    snapshots: Vec<String>,
}

pub fn q16b_failing() {
    // the best way is to split the workload into 2 halves with power set, then evaluate the score of set 1 + set 2.
    let vec = read_to_lines("inp_q16.txt");

    // read data into Valve struct
    let mut valves: HashMap<String, Valve> = HashMap::new();
    let re = Regex::new(r"Valve ([A-Z]+) has flow rate=(\d+); tunnels* leads* to valves* ((?:(?:\w+)(?:, )?)*)").unwrap();

    for line in vec.into_iter() {
        let groups = re.captures(&line.as_str()).unwrap();
        
        let code = groups.get(1).map(|x| x.as_str().to_string()).unwrap();
        let flow_rate = groups.get(2).map(|x| x.as_str().parse::<i32>().unwrap()).unwrap();
        let adjacent = groups.get(3).map(|x| x.as_str().split(", ").map(|x| x.to_string()).collect::<Vec<String>>()).unwrap();
        let valve = Valve {code: code.clone(), flow_rate, adjacent} ;
        valves.insert(code, valve);
    }

    let mut distance: HashMap<(String, String), i32> = HashMap::new();

    for start in valves.keys() {
        for end in valves.keys() {
            if start == end { continue; }
            distance.insert((start.clone(), end.clone()), precompute_path(start, end, &valves));
            // println!("{}->{}: {} steps", start, end, precompute_path(start, end, &valves));
        }
    }

    valves.insert("__".to_string(), Valve { code: "__".to_string(), flow_rate: 0, adjacent: Vec::new() } );

    // search
    let mut all_valves = valves.keys().cloned().filter(|x| x != &"AA".to_string()).collect::<Vec<String>>();
    let mut nonzero_valves = valves.iter().filter(|(_, v)| v.flow_rate > 0).map(|(k, _)| k.to_string()).collect::<Vec<String>>();
    all_valves.sort();
    nonzero_valves.sort();

    // let mut frontier: Vec<Agenda> = Vec::new();
    let mut frontier = PriorityQueue::new();
    // let mut best: HashMap<(String, String, Vec<String>), i32> = HashMap::new();
    let mut best: HashMap<(Vec<String>, i32), i32> = HashMap::new();
    let mut max_relief = 0;

    for dest_i in nonzero_valves.iter() {
        for dest_e in nonzero_valves.iter() {
            
            if dest_i == dest_e { continue; }

            let dist_i = distance.get(&("AA".to_string(), dest_i.to_string())).unwrap();
            let dist_e = distance.get(&("AA".to_string(), dest_e.to_string())).unwrap();

            let remaining_new = nonzero_valves.iter()
                                                    .filter(|&x| x != dest_i && x != dest_e)
                                                    .map(|x| x.to_string())
                                                    .collect::<Vec<String>>();

            let agenda = AgendaDuo { 
                pos_i: "AA".to_string(),
                pos_e: "AA".to_string(),
                goal_i: dest_i.to_string(), 
                goal_e: dest_e.to_string(),
                time: 26, 
                time_i: 26 - dist_i - 1,
                time_e: 26 - dist_e - 1,
                acc_flow_rate: 0, 
                acc_relief: 0, 
                opened: Vec::new(), 
                visited_i: Vec::new(), 
                visited_e: Vec::new(), 
                remaining: remaining_new,
                snapshots: Vec::new(),
            };

            frontier.push(agenda, valves.get(dest_i).unwrap().flow_rate + valves.get(dest_e).unwrap().flow_rate);
        }
    }

    let mut eval_count = 0;

    'outer: while frontier.len() > 0 {
        eval_count += 1;
        let (agenda, _) = frontier.pop().unwrap();
        // let agenda_old = agenda.clone();

        // done
        if agenda.goal_i == "__".to_string() 
        && agenda.goal_e == "__".to_string()
        && agenda.remaining.len() == 0
        || agenda.time <= 0 {
            let relief = agenda.acc_relief + agenda.acc_flow_rate * agenda.time;
            if relief > max_relief {
                println!("\n.. max after #cmp={eval_count}, agenda={:?}", agenda);
                max_relief = relief;
            }
            continue;
        }

        if agenda.goal_i == "__".to_string() {

            for (ix, dest) in agenda.remaining.iter().enumerate() {
                let dist = distance.get(&(agenda.pos_i.to_string(), dest.to_string())).unwrap();
                let time_spent = dist + 1;

                let mut remaining_new = agenda.remaining.clone();
                remaining_new.remove(ix);

                let agenda_new = AgendaDuo { 
                    pos_i: agenda.pos_i.to_string(),
                    pos_e: agenda.pos_e.to_string(),
                    goal_i: dest.to_string(), 
                    goal_e: agenda.goal_e.to_string(),
                    time: agenda.time, 
                    time_i: agenda.time - time_spent,
                    time_e: agenda.time_e,
                    acc_flow_rate: agenda.acc_flow_rate, 
                    acc_relief: agenda.acc_relief, 
                    opened: agenda.opened.clone(), 
                    visited_i: agenda.visited_i.clone(), 
                    visited_e: agenda.visited_e.clone(), 
                    remaining: remaining_new,
                    snapshots: agenda.snapshots.clone()
                };

                let prio = &agenda_new.acc_relief + &agenda_new.acc_flow_rate * &agenda_new.time;
                frontier.push(agenda_new, prio.clone());
            }

        } else if agenda.goal_e == "__".to_string() {

            for (ix, dest) in agenda.remaining.iter().enumerate() {
                let dist = distance.get(&(agenda.pos_e.to_string(), dest.to_string())).unwrap();
                let time_spent = dist + 1;

                let mut remaining_new = agenda.remaining.clone();
                
                remaining_new.remove(ix);

                let agenda_new = AgendaDuo { 
                    pos_i: agenda.pos_i.to_string(), 
                    pos_e: agenda.pos_e.to_string(), 
                    goal_i: agenda.goal_i.to_string(),
                    goal_e: dest.to_string(),
                    time: agenda.time, 
                    time_i: agenda.time_i,
                    time_e: agenda.time - time_spent,
                    acc_flow_rate: agenda.acc_flow_rate, 
                    acc_relief: agenda.acc_relief, 
                    opened: agenda.opened.clone(), 
                    visited_i: agenda.visited_i.clone(), 
                    visited_e: agenda.visited_e.clone(), 
                    remaining: remaining_new,
                    snapshots: agenda.snapshots.clone()
                };

                let prio = &agenda_new.acc_relief + &agenda_new.acc_flow_rate * &agenda_new.time;
                frontier.push(agenda_new, prio.clone());
            }

        } else {
            let mut opened_new = agenda.opened.clone();
            let mut snapshots_new = agenda.snapshots.clone();
            let mut visited_i_new = agenda.visited_i.clone();
            let mut visited_e_new = agenda.visited_e.clone();
            
            let time_new = cmp::max(agenda.time_i, agenda.time_e);

            if time_new <= 0 {
                let agenda_new = AgendaDuo { 
                    pos_i: agenda.pos_i, 
                    pos_e: agenda.pos_e, 
                    goal_i: agenda.goal_i, 
                    goal_e: agenda.goal_e, 
                    time: 0, 
                    time_i: agenda.time_i,
                    time_e: agenda.time_e,
                    acc_flow_rate: agenda.acc_flow_rate,
                    acc_relief: agenda.acc_relief + agenda.time * agenda.acc_flow_rate,
                    opened: opened_new, 
                    visited_i: visited_i_new,
                    visited_e: visited_e_new, 
                    remaining: agenda.remaining,
                    snapshots: snapshots_new,
                };
                
                let prio = &agenda_new.acc_relief + &agenda_new.acc_flow_rate * &agenda_new.time;
                frontier.push(agenda_new, prio);
                continue;
            }

            if time_new == agenda.time_i { 
                visited_i_new.push(agenda.goal_i.clone());
                opened_new.push(agenda.goal_i.clone());
                let s = format!("open {} at t={}, flow rate={}", agenda.goal_i, time_new, valves.get(&agenda.goal_i).unwrap().flow_rate);
                snapshots_new.push(s);
            }
            if time_new == agenda.time_e { 
                visited_e_new.push(agenda.goal_e.clone());
                opened_new.push(agenda.goal_e.clone());
                let s = format!("open {} at t={}, flow rate={}", agenda.goal_e, time_new, valves.get(&agenda.goal_e).unwrap().flow_rate);
                snapshots_new.push(s);
            }

            opened_new.sort();

            let agenda_new = AgendaDuo { 
                pos_i: if time_new == agenda.time_i { agenda.goal_i.clone() } else { agenda.pos_i.clone() }, 
                pos_e: if time_new == agenda.time_e { agenda.goal_e.clone() } else { agenda.pos_e.clone() }, 
                goal_i: if time_new == agenda.time_i { "__".to_string() } else { agenda.goal_i.clone() }, 
                goal_e: if time_new == agenda.time_e { "__".to_string() } else { agenda.goal_e.clone() }, 
                time: time_new, 
                time_i: if time_new == agenda.time_i { -1 } else { agenda.time_i.clone() },
                time_e: if time_new == agenda.time_e { -1 } else { agenda.time_e.clone() },
                acc_flow_rate: agenda.acc_flow_rate 
                                + if time_new == agenda.time_i { valves.get(&agenda.goal_i).unwrap().flow_rate } else { 0 }
                                + if time_new == agenda.time_e { valves.get(&agenda.goal_e).unwrap().flow_rate } else { 0 }, 
                acc_relief: agenda.acc_relief 
                            + (agenda.time - time_new) * agenda.acc_flow_rate,
                opened: opened_new, 
                visited_i: visited_i_new,
                visited_e: visited_e_new, 
                remaining: agenda.remaining,
                snapshots: snapshots_new,
            };

            // let key = (agenda_new.pos_i.clone(), agenda_new.pos_e.clone(), agenda_new.opened.clone());
            let key = (agenda_new.remaining.clone(), agenda_new.time.clone());
            let &incum = best.get(&key).unwrap_or(&-1);
            let prio = &agenda_new.acc_relief + &agenda_new.acc_flow_rate * &agenda_new.time;
        
            if &prio >= &incum {
                best.insert(key, prio.clone());
                frontier.push(agenda_new, prio);
            }

            // let prio = &agenda_new.acc_relief + &agenda_new.acc_flow_rate * &agenda_new.time;
            // frontier.push(agenda_new, prio);
        }
    }

    println!("q16b: {}, eval_count={}", max_relief, eval_count);
}
