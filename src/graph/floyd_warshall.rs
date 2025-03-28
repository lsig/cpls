use std::cmp;

pub fn floyd_warshall(adj: Vec<Vec<isize>>) -> Vec<Vec<isize>> {
    let n = adj.len();
    let mut dist: Vec<Vec<isize>> = vec![vec![0; n]; n];

    for i in 0..n {
        for j in 0..n {
            if i == j {
                dist[i][j] = 0;
            } else if adj[i][j] != 0 {
                dist[i][j] = adj[i][j];
            } else {
                dist[i][j] = isize::MAX;
            }
        }
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if let Some(d) = dist[i][k].checked_add(dist[k][j]) {
                    dist[i][j] = cmp::min(dist[i][j], d);
                }
            }
        }
    }

    dist
}
