use anyhow::{anyhow, Error as AnyhowError};
use petgraph::{
    algo::floyd_warshall,
    dot::{Config, Dot},
    stable_graph::NodeIndex,
    Graph,
};
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs,
};

#[derive(Debug)]
struct Valve {
    id: String,
    flow: usize,
    tunnel_to: Vec<String>,
}

fn parse_graph(input: String) -> Result<(Graph<(String, usize), usize>, NodeIndex), AnyhowError> {
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

    let mut graph = Graph::new();
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
                    graph.add_edge(a, b, 1);
                }
            }
        }
    }

    graph.shrink_to_fit();

    println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));

    let start = *graph_map.get("AA").ok_or(anyhow!("No start"))?;

    Ok((graph, start))
}

fn follow_path(
    graph: &Graph<(String, usize), usize>,
    dists: &HashMap<(NodeIndex, NodeIndex), usize>,
    start: NodeIndex,
    time_limit: usize,
    visited: &mut HashSet<NodeIndex>,
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
                let (path_gain, sub_path) =
                    follow_path(graph, dists, end, time_limit - time_to_open, visited);

                let total_gain = path_gain + gain;

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
    graph: &Graph<(String, usize), usize>,
    dists: &HashMap<(NodeIndex, NodeIndex), usize>,
    start: NodeIndex,
) -> Result<(), AnyhowError> {
    let (max_gain, mut path) = follow_path(&graph, &dists, start, 30, &mut HashSet::new());

    path.reverse();

    println!("Max gain: {}", max_gain);
    println!("Path: {:?}", path);

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./input_simple.txt")?;

    let (graph, start) = parse_graph(input)?;

    let dists = floyd_warshall(&graph, |e| *e.weight()).map_err(|_| anyhow!("Negative cycles"))?;

    part_1(&graph, &dists, start)?;

    Ok(())
}
