use anyhow::{anyhow, Error as AnyhowError};
use petgraph::{
    algo::dijkstra, dot::Dot, prelude::UnGraph, stable_graph::NodeIndex, visit::IntoNodeReferences,
    Direction, Graph,
};
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fmt::Display,
    fs,
};

#[derive(Debug)]
struct Valve {
    id: String,
    flow: usize,
    tunnel_to: Vec<String>,
}

fn simplify(graph: &mut UnGraph<(String, usize), usize>) {
    loop {
        let mut to_remove: Option<NodeIndex> = None;
        for (i, n) in graph.node_references() {
            if n.0 != "AA" && n.1 == 0 {
                to_remove = Some(i);
                break;
            }
        }
        if to_remove.is_none() {
            break;
        } else if let Some(to_remove) = to_remove {
            let mut to_add = HashSet::new();

            let mut incoming = graph.neighbors_directed(to_remove, Direction::Incoming);
            let mut outgoing = graph.neighbors_directed(to_remove, Direction::Outgoing);

            while let Some(a) = incoming.next() {
                while let Some(b) = outgoing.next() {
                    if a != b {
                        if let Some(e1) = graph.find_edge(a, to_remove) {
                            if let Some(e2) = graph.find_edge(to_remove, b) {
                                if let Some(w1) = graph.edge_weight(e1) {
                                    if let Some(w2) = graph.edge_weight(e2) {
                                        to_add.insert((a, b, w1 + w2));
                                    }
                                }
                            }
                        }
                    }
                }
            }

            for (a, b, weight) in to_add {
                graph.add_edge(a, b, weight);
            }

            graph.remove_node(to_remove);
        }
    }

    graph.shrink_to_fit();
}

fn parse_graph(
    input: String,
) -> Result<
    (
        UnGraph<(String, usize), usize>,
        NodeIndex,
        HashMap<String, NodeIndex>,
    ),
    AnyhowError,
> {
    let regex = Regex::new(r"Valve ([A-Z]+) has flow rate=(\d+); tunnels? leads? to valves? (.*)")?;

    let valves = input
        .lines()
        .filter_map(|s| regex.captures(s))
        .filter_map(|cap| {
            Some(Valve {
                id: cap.get(1)?.as_str().to_owned(),
                flow: cap.get(2)?.as_str().parse().ok()?,
                tunnel_to: cap
                    .get(3)?
                    .as_str()
                    .split(", ")
                    .map(|s| s.to_owned())
                    .collect(),
            })
        })
        .collect::<Vec<_>>();

    let valve_map = HashMap::<String, &Valve>::from_iter(valves.iter().map(|v| (v.id.clone(), v)));

    let mut graph = Graph::new_undirected();
    let mut graph_map: HashMap<String, NodeIndex> = HashMap::new();

    for v in &valves {
        let mut a = graph_map.get(&v.id).cloned();
        if a.is_none() {
            let flow = valve_map.get(&v.id).map(|v| v.flow);

            let i = graph.add_node((v.id.clone(), flow.unwrap_or(0)));

            graph_map.insert(v.id.clone(), i);
            a = Some(i);
        }

        for id in &v.tunnel_to {
            let mut b = graph_map.get(id).cloned();
            if b.is_none() {
                let flow = valve_map.get(id).map(|v| v.flow);

                let i = graph.add_node((id.clone(), flow.unwrap_or(0)));

                graph_map.insert(id.clone(), i);
                b = Some(i);
            }

            if let Some(a) = a {
                if let Some(b) = b {
                    if !graph.contains_edge(a, b) {
                        graph.add_edge(a, b, 1);
                    }
                }
            }
        }
    }

    simplify(&mut graph);

    graph_map.clear();
    for i in graph.node_indices() {
        graph_map.insert(graph[i].0.clone(), i);
    }

    println!("{:?}", Dot::with_config(&graph, &[]));

    let start = *graph_map.get("AA").ok_or(anyhow!("No start"))?;

    Ok((graph, start, graph_map))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SeekFor {
    Total,
    Node,
}

fn follow_path(
    graph: &UnGraph<(String, usize), usize>,
    dists: &HashMap<(NodeIndex, NodeIndex), usize>,
    start: NodeIndex,
    time_limit: usize,
    visited: &mut HashSet<NodeIndex>,
    level: usize,
    seek_for: SeekFor,
) -> (usize, Vec<String>) {
    if time_limit == 0 {
        return (0, vec![graph[start].0.clone()]);
    }

    let dests = graph.node_indices().into_iter();

    let mut max_gain = 0;
    let mut path = vec![graph[start].0.clone()];

    for end in dests {
        let dest_flow = graph[end].1;

        if dest_flow > 0 && !visited.contains(&end) {
            if let Some(dist) = dists.get(&(start, end)) {
                let time_to_open = dist + 1;

                if time_to_open > time_limit {
                    continue;
                }

                let gain = (time_limit - time_to_open) * dest_flow;

                let backtrack_to = visited.clone();

                visited.insert(end);
                let (path_gain, sub_path) = follow_path(
                    graph,
                    dists,
                    end,
                    time_limit - time_to_open,
                    visited,
                    level + 1,
                    seek_for,
                );

                let total_gain = if seek_for == SeekFor::Total {
                    path_gain + gain
                } else {
                    gain
                };

                // if level == 0 {
                //     println!("Consider {:?} = {}", &graph[end], total_gain);
                // }

                if total_gain > max_gain {
                    max_gain = total_gain;
                    path = sub_path.clone();
                    path.push(graph[start].0.clone());
                }

                visited.clear();
                for n in backtrack_to {
                    visited.insert(n);
                }
            }
        }
    }

    (max_gain, path)
}

fn part_1(
    graph: &UnGraph<(String, usize), usize>,
    dists: &HashMap<(NodeIndex, NodeIndex), usize>,
    start: NodeIndex,
) -> Result<(), AnyhowError> {
    let (max_gain, mut path) = follow_path(
        &graph,
        &dists,
        start,
        30,
        &mut HashSet::new(),
        0,
        SeekFor::Total,
    );

    path.reverse();

    println!("Max gain: {}", max_gain);
    println!("Path: {:?}", path);
    println!();

    Ok(())
}

#[derive(Debug)]
struct Agent<'a> {
    name: String,
    time_left: usize,
    total_gain: usize,
    position: NodeIndex,
    graph: &'a UnGraph<(String, usize), usize>,
}

impl<'a> Display for Agent<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} open {} (total gain {}, time remaining {})",
            self.name, self.graph[self.position].0, self.total_gain, self.time_left
        )
    }
}

fn part_2(
    graph: &UnGraph<(String, usize), usize>,
    dists: &HashMap<(NodeIndex, NodeIndex), usize>,
    start: NodeIndex,
    graph_map: &HashMap<String, NodeIndex>,
) -> Result<(), AnyhowError> {
    let mut scheduled = HashSet::new();

    let agents = &mut [
        Agent {
            name: "You".to_owned(),
            time_left: 26,
            total_gain: 0,
            position: start,
            graph,
        },
        Agent {
            name: "Elephant".to_owned(),
            time_left: 26,
            total_gain: 0,
            position: start,
            graph,
        },
    ];

    for _ in 0..100 {
        agents.sort_by(|a, b| b.time_left.cmp(&a.time_left));

        for mut agent in agents.iter_mut() {
            let (_path_gain, path) = follow_path(
                graph,
                dists,
                agent.position,
                agent.time_left,
                &mut scheduled,
                0,
                SeekFor::Node,
            );

            if path.len() < 2 {
                continue;
            }

            let go_to = &path[path.len() - 2];
            let idx = graph_map.get(go_to).cloned().ok_or(anyhow!("Wrong node"))?;
            scheduled.insert(idx);

            let dist = dists.get(&(agent.position, idx)).ok_or(anyhow!(
                "No distance: {} -> {}",
                graph[agent.position].0,
                graph[idx].1
            ))?;

            if dist + 1 > agent.time_left {
                continue;
            }

            agent.time_left -= dist + 1;
            agent.position = idx;
            agent.total_gain += agent.time_left * &graph[idx].1;

            println!("{}", agent);
            println!();
        }
    }

    println!(
        "Total gain: {}",
        &agents.iter().map(|a| a.total_gain).sum::<usize>()
    );

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./input_simple.txt")?;

    let (graph, start, graph_map) = parse_graph(input)?;

    let mut dists = HashMap::new();

    for n in graph.node_indices() {
        let d = dijkstra(&graph, n, None, |e| *e.weight());
        for (k, v) in d.iter() {
            dists.insert((n, *k), *v);
        }
    }

    part_1(&graph, &dists, start)?;
    part_2(&graph, &dists, start, &graph_map)?;

    Ok(())
}
