use std::cmp::{min,max};
use std::collections::{HashMap,HashSet};

pub fn output_graph(input:&str) -> String {
    String::from("digraph graphName {\n".to_string()
        + &input.split("\n")
            .flat_map(|l|{
                let parts:Vec<_> = l.split(": ").collect();
                let connectors:Vec<_> = parts[1].split(" ").collect();
                connectors.iter().map(|c|String::from(format!("  {} -> {} [dir=both]", parts[0], c))).collect::<Vec<_>>()
            }).collect::<Vec<_>>().join("\n").to_string()
        + "\n}" )
}

type NodeMap = HashMap<String, HashSet<String>>;
type NodeStepMap = HashSet<(String, String)>;

pub fn find_bridges_product(input:&str) -> usize {
    let mut nodes:NodeMap = HashMap::new();

    // Parse Nodes
    for line in input.split("\n").collect::<Vec<_>>() {
        let parts:Vec<_> = line.split(": ").collect();
        let node = parts[0].to_string();
        let connectors:Vec<_> = parts[1].split(" ").collect();
        let mut map = match nodes.contains_key(&node) {
            true => nodes.get(&node).unwrap().clone(),
            false => HashSet::new()
        };
        for connector in connectors {
            let mut cm = HashSet::new();
            if let Some(connector_map) = nodes.get(connector) {
                cm = connector_map.clone();
            }
            map.insert(connector.to_string());
            cm.insert(node.clone());
            nodes.insert(connector.to_string(), cm.clone());
        }
        nodes.insert(node.clone(), map.clone());
    }

    let node_keys:Vec<&String> = nodes.keys().into_iter().collect::<Vec<_>>().clone();
    let mut paths:HashSet<(String, String)> = HashSet::new();
    // Determine BFS sets
    for x in 0..node_keys.len() {
        for y in (x + 1)..node_keys.len() {
            paths.insert((node_keys[x].to_string(), node_keys[y].to_string()));
        }
    }
    // use rand::Rng;
    // while paths.len() < 30 {
    //     let (n1,n2) = (rand::thread_rng().gen_range(0..node_keys.len()), rand::thread_rng().gen_range(0..node_keys.len()));
    //     let (p1,p2) = (node_keys[n1], node_keys[n2]);
    //     paths.insert((
    //         std::cmp::min(p1.to_string(),p2.to_string()),
    //         std::cmp::max(p1.to_string(),p2.to_string())
    //     ));
    // }

    let mut memo:NodePathMemo = NodePathMemo::new();
    let npath = paths.len();
    let mut curr = 0;
    let mut visited_sets:Vec<HashSet<(String, String)>> = Vec::new();
    
    for p in paths {
        curr += 1;
        println!("{}/{} ({})", curr, npath, memo.dict.len());
        let visited_set = traverse_bfs(&p.0, &p.1, &nodes, &mut memo);
        // println!("{} -> {}: Found set: {:?}", p.0, p.1, visited_set);
        visited_sets.push(visited_set);

        let (left, right) = count_sets(&visited_sets, &nodes);
        if left * right > 0 {
            return left * right
        }
    }

    0
}

fn count_sets(visited_sets:&Vec<HashSet<(String, String)>>, nodes:&NodeMap) -> (usize, usize) {
    let mut link_counts:HashMap<(String, String), usize> = HashMap::new();
    for visited_set in visited_sets {
        for link in visited_set {
            *link_counts.entry(link.to_owned()).or_default() += 1;
        }
    }

    use std::collections::BTreeMap;
    let count_b: BTreeMap<&usize,&(String,String)> = link_counts.iter().map(|(k,v)| (v,k)).collect();

    let bridges:Vec<(String, String)> = count_b.into_iter().rev().take(3).map(|(_,(s,f))|(s.to_string(), f.to_string())).collect();
    println!("Found {:?}", bridges);

    let start = bridges[0].1.to_string();
    let left = nodes.len() - count_loop(start, &nodes, &bridges);
    let right = nodes.len() - left;
    println!("left {}", left);
    println!("right {}", right);
    (left, right)
}

fn count_loop(start:String, nodes:&NodeMap, bridges:&Vec<(String, String)>) -> usize {
    let mut nodes = nodes.clone();
    for (left, right) in bridges {
        let left_connections = nodes.get_mut(left).unwrap();
        left_connections.remove(right);
        let right_connections = nodes.get_mut(right).unwrap();
        right_connections.remove(left);
    }
    let mut frontier = vec!(start);
    let mut visited:HashSet<String> = HashSet::new();
    while let Some(pos) = frontier.pop() {
        if visited.insert(pos.to_string()) {
            let links = nodes.get(&pos).unwrap();
            for connection in links {
                frontier.push(connection.clone());
            }
        }
    }
    
    visited.len()
}

fn traverse_bfs(start:&str, end:&str, nodes:&NodeMap, memo:&mut NodePathMemo) -> HashSet<(String, String)> {
    if let Some(results) = memo.get(start.to_string(), end.to_string()) {
        println!("Cache hit!");
        return results;
    }
    use std::collections::BinaryHeap;
    let mut frontier = BinaryHeap::new();
    frontier.push(NodeStep{
        node: start.to_string(),
        visited: HashSet::new(),
        links: Vec::new(),
        count: 0
    });
    while let Some(step) = frontier.pop() {
        let mut visited = step.visited.clone();
        if visited.insert(step.node.clone()) {
            let links = step.links.clone();
            if step.node == end {
                for x in 0..links.len() {
                    for y in x..links.len() {
                        let (link_a,link_b) = (links[x].clone(), links[y].clone());
                        let (a, _) = link_a;
                        let (_, b) = link_b;
                        let linko:Vec<(String,String)> = links[x..=y].iter().map(|(c,d)|(min(c.clone(),d.clone()),max(c.clone(),d.clone()))).collect();
                        let result = HashSet::from_iter(linko.into_iter());
                        if let Some(memoized) = memo.get(a.clone(),b.clone()) {
                            if result.len() < memoized.len() {
                                // if a == "nvd" || b == "nvd" {
                                //     println!("memoizing {} - {} as {:?}", a, b, result);
                                // }
                                memo.set(a.clone(),b.clone(),result.clone());
                            }
                        } else {
                            // if a == "nvd" || b == "nvd" {
                            //     println!("memoizing {} - {} as {:?}", a, b, result);
                            // }
                            memo.set(a.clone(),b.clone(),result.clone());
                        }
                    }
                }
                
                let links_massaged:Vec<(String,String)> = step.links.into_iter().map(|(a,b)|(min(a.clone(),b.clone()),max(a.clone(),b.clone()))).collect();
                let result = HashSet::from_iter(links_massaged.into_iter());
                memo.set(start.to_string(),end.to_string(), result.clone());
                return result;
            }

            if let Some(connections) = nodes.get(&step.node) {
                for connection in connections {
                    let mut links = step.links.clone();
                    links.push((step.node.clone(), connection.to_string()));

                    frontier.push(NodeStep{
                        node: connection.to_string(),
                        links: links,
                        visited: visited.clone(),
                        count: step.count + 1
                    });
                }
            }
        }
    }
    return HashSet::new();
}

struct NodePathMemo {
    dict:HashMap<(String,String), NodeStepMap>,
}

impl NodePathMemo {
    fn new() -> NodePathMemo {
        NodePathMemo {
            dict: HashMap::new()
        }
    }
    fn get(&self, start:String, end:String) -> Option<NodeStepMap> {
        self.dict.get(&(start,end)).cloned()
    }

    fn set(&mut self, start:String, end:String, visited:NodeStepMap) {
        self.dict.insert((start.to_string(), end.to_string()), visited.clone());
    }
}

#[derive(Eq, PartialEq)]
struct NodeStep {
    node:String,
    visited:HashSet<String>,
    links:Vec<(String,String)>,
    count:usize,
}

impl PartialOrd for NodeStep {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NodeStep {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // We are using a min heap, so we are doing this backwards.
        other
            .count
            .cmp(&self.count)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_bridges_product() {
        let input = "jqt: rhn xhk nvd\nrsh: frs pzl lsr\nxhk: hfx\ncmg: qnr nvd lhk bvb\nrhn: xhk bvb hfx\nbvb: xhk hfx\npzl: lsr hfx nvd\nqnr: nvd\nntq: jqt hfx bvb xhk\nnvd: lhk\nlsr: lhk\nrzs: qnr cmg lsr rsh\nfrs: qnr lhk lsr";
        assert_eq!(54, find_bridges_product(input));
    }
}