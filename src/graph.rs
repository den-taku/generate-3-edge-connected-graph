//! Fundamental methods and definitions for Graph

use itertools::Itertools;
use rand::Rng;
use std::cmp::{max, min};
use std::collections::HashSet;
use std::convert::identity;

//--------------------------------------------------------------------------------------------------
// Submodules
//--------------------------------------------------------------------------------------------------

//--------------------------------------------------------------------------------------------------
// Private Definitions
//--------------------------------------------------------------------------------------------------

//--------------------------------------------------------------------------------------------------
// Private Code
//--------------------------------------------------------------------------------------------------

//--------------------------------------------------------------------------------------------------
// Public Definitions
//--------------------------------------------------------------------------------------------------

/// Adjacent matrix of undirected graph
#[derive(Clone, Debug)]
pub struct Graph(Vec<Vec<usize>>);

//--------------------------------------------------------------------------------------------------
// Public Code
//--------------------------------------------------------------------------------------------------

/// generate simple undirected graph
pub fn generate_random_graph_edges(nodes: usize, edges: usize) -> HashSet<(usize, usize)> {
    if nodes == 0 {
        // empty graph
        return HashSet::new();
    } else if edges >= nodes * (nodes - 1) / 2 {
        // complete graph
        let mut all = HashSet::new();
        for u in 0..nodes {
            for v in u + 1..nodes {
                all.insert((u, v));
            }
        }
        return all;
    }
    let mut ret = HashSet::with_capacity(edges);

    let mut rng = rand::thread_rng();
    while ret.len() < edges {
        let u = rng.gen::<usize>() % nodes;
        let v = rng.gen::<usize>() % nodes;
        if u != v {
            let edge = (min(u, v), max(u, v));
            ret.insert(edge);
        }
    }
    ret
}

pub fn enumerate_k_edge_connected_induced_subgraphs<
    T: std::iter::IntoIterator<Item = (usize, usize)> + Clone,
>(
    nodes: usize,
    edges: T,
    k: usize,
) {
    for mut sub in (0..nodes).powerset() {
        sub.sort();
        // O(n^2) time needed but maybe invokes no problem
        let compressed: Vec<_> = (0..nodes)
            .map(|i| {
                sub.iter()
                    .enumerate()
                    .find(|(_, j)| i == **j)
                    .map(|(index, _)| index)
            })
            .collect();
        let subgraph = Graph::new(
            sub.len(),
            edges
                .clone()
                .into_iter()
                .map(|(u, v)| {
                    let u_comp = compressed[u];
                    let v_comp = compressed[v];
                    if let Some(u) = u_comp {
                        if let Some(v) = v_comp {
                            Some((u, v))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .flatten(),
        );
        if subgraph.is_k_edge_connected(k) {
            println!("  subgraphs: {sub:?} is {}-edge-connected.", k);
        }
    }
}

impl Graph {
    pub fn new<T: std::iter::IntoIterator<Item = (usize, usize)>>(nodes: usize, edges: T) -> Self {
        let mut adjacent = vec![Vec::new(); nodes];
        for (u, v) in edges {
            // undirected
            adjacent[u].push(v);
            adjacent[v].push(u);
        }
        Self(adjacent)
    }

    pub fn is_connected(&self) -> bool {
        if self.0.is_empty() {
            return true;
        }
        let mut visited = vec![false; self.0.len()];
        visited[0] = true;
        self.dfs(0, &mut visited, &|_| {}, &|_| {});
        visited.into_iter().all(identity)
    }

    pub fn is_k_edge_connected(&self, k: usize) -> bool {
        let mut edges = HashSet::new();
        for (u, vs) in self.0.iter().enumerate() {
            for &v in vs {
                let edge = (min(u, v), max(u, v));
                edges.insert(edge);
            }
        }
        if edges.len() <= k {
            return false;
        }
        for sub in edges.clone().into_iter().combinations(edges.len() - k) {
            if !Graph::new(self.0.len(), sub).is_connected() {
                return false;
            }
        }
        true
    }

    /// BFS
    ///
    /// - condition: before visiting `start`, you should make the start `true`
    pub fn dfs<F: Fn(usize), U: Fn(usize)>(
        &self,
        start: usize,
        visited: &mut [bool],
        before_visited: &F,
        after_visited: &U,
    ) {
        before_visited(start);
        for &v in &self.0[start] {
            if !visited[v] {
                visited[v] = true;
                self.dfs(v, visited, before_visited, after_visited);
            }
        }
        after_visited(start);
    }
}

//--------------------------------------------------------------------------------------------------
// Test
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {}
