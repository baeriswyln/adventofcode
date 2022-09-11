use std::collections::HashMap;

struct Node {
    name: String,
    index: u8,
    single_visit: bool,
    links: Vec<String>,
}

pub struct PassagePathing {
    nodes: HashMap<String, Node>,
}

impl crate::Advent for PassagePathing {
    fn new(data: &str) -> Self {
        let mut nodes: HashMap<String, Node> = HashMap::new();

        data.lines().for_each(|l| {
            let mut parts = l.split("-").into_iter();
            let start = parts.next().unwrap().to_string();
            let end = parts.next().unwrap().to_string();
            let mut len = nodes.len();

            nodes.entry(start.clone())
                .and_modify(|e| e.links.push(end.clone()))
                .or_insert(Node {
                    name: start.clone(),
                    index: len as u8,
                    single_visit: start.chars().nth(0).unwrap().is_lowercase(),
                    links: vec![end.clone()],
                });

            len = nodes.len();
            nodes.entry(end.clone())
                .and_modify(|e| e.links.push(start.clone()))
                .or_insert(Node {
                    name: end.clone(),
                    index: len as u8,
                    single_visit: end.chars().nth(0).unwrap().is_lowercase(),
                    links: vec![start.clone()],
                });
        });

        PassagePathing {
            nodes
        }
    }

    fn year() -> u16 { 2021 }

    fn day() -> u8 { 12 }

    fn part1(&self) -> usize {
        let starting_node = self.nodes.get("start").unwrap();
        get_nbr_of_paths(starting_node, String::from("start"), &self.nodes)
    }

    fn part2(&self) -> usize {
        let starting_node = self.nodes.get("start").unwrap();
        get_nbr_of_paths_extended(starting_node, vec![starting_node.index], &self.nodes, false)
    }
}

fn get_nbr_of_paths(node: &Node, path: String, node_list: &HashMap<String, Node>) -> usize {
    let mut result = 0;
    if node.name.eq("end") { return 1; }

    for link in node.links.iter() {
        let next_node = node_list.get(link).unwrap();
        let l = format!("-{}", link);
        if next_node.single_visit && path.contains(&l) { continue; }
        if link.eq("start") { continue; }

        let new_path = format!("{}-{}", path, link);
        result += get_nbr_of_paths(next_node, new_path, node_list);
    }

    result
}

fn get_nbr_of_paths_extended(node: &Node, path: Vec<u8>, node_list: &HashMap<String, Node>, visited_twice: bool) -> usize {
    let mut result = 0;
    if node.name.eq("end") { return 1; }

    for link in node.links.iter() {
        let next_node = node_list.get(link).unwrap();
        if link.eq("start") { continue; }
        if next_node.single_visit && path.contains(&next_node.index) {
            if !visited_twice {
                let temp = vec![next_node.index];
                let new_path = vec![&path[..], &temp].concat();
                result += get_nbr_of_paths_extended(next_node, new_path, node_list, true);
            }
            continue;
        }

        let temp = vec![next_node.index];
        let new_path = vec![&path[..], &temp].concat();
        result += get_nbr_of_paths_extended(next_node, new_path, node_list, visited_twice);
    }

    result
}