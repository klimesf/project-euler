use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs;

pub(crate) fn e79() {
    println!("{}", passcode_derivation(fs::read_to_string("input/e79/input.txt").unwrap()))
}

fn passcode_derivation(input: String) -> String {
    let mut edges = HashMap::new();
    let mut vertices = HashSet::new();
    input.lines().for_each(|line| {
        line.chars().tuple_windows().for_each(|(first, second)| {
            vertices.insert(first);
            vertices.insert(second);
            edges.entry(first).or_insert_with(HashSet::new).insert(second);
        });
    });
    topology_sort(&edges, &vertices)
}

fn topology_sort(edges: &HashMap<char, HashSet<char>>, vertices: &HashSet<char>) -> String {
    let mut in_degree = HashMap::new();
    for vertex in vertices {
        in_degree.insert(vertex, 0);
    }
    for incoming in edges.values() {
        for to in incoming {
            *in_degree.entry(to).or_insert(0) += 1;
        }
    }

    let mut ans = String::new();
    while !in_degree.is_empty() {
        let remove = *in_degree.iter().find(|(_, v)| **v == 0).unwrap().0;
        ans.push(*remove);

        if let Some(incoming) = edges.get(&remove) {
            for to in incoming {
                *in_degree.entry(to).or_insert(0) -= 1;
            }
        }
        in_degree.remove(remove);
    }
    ans
}

#[cfg(test)]
mod e79_tests {
    use crate::e79::passcode_derivation;
    use std::fs;

    #[test]
    fn max_path_sum_works() {
        assert_eq!("73162890", passcode_derivation(fs::read_to_string("input/e79/test.txt").unwrap()));
    }
}
