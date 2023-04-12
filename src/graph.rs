//! Fundamental methods and definitions for Graph

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

/// generate sinple undirected graph
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
mod tests {
}
