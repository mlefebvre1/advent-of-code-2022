use super::shortest_path::ShortestPath;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Edge {
    pub src: usize,
    pub dst: usize,
    pub weight: isize,
}

#[derive(Debug, Clone)]
pub struct Vertex {
    pub id: usize,
    pub neighbors: Vec<Edge>,
}

#[derive(Debug)]
pub struct Dgraph {
    pub vertices: Vec<Vertex>,
    pub edges: Vec<Edge>,
}

impl Dgraph {
    pub fn new(nb_vertices: usize) -> Self {
        let mut graph = Self {
            vertices: Vec::new(),
            edges: Vec::new(),
        };
        for _ in 0..nb_vertices {
            graph.add_vertex();
        }
        graph
    }

    pub fn add_vertex(&mut self) {
        let vertex = Vertex {
            id: self.vertices.len(),
            neighbors: Vec::new(),
        };
        self.vertices.push(vertex);
    }

    pub fn add_edge(&mut self, edge: Edge) {
        self.edges.push(edge);
        self.vertices[edge.src].neighbors.push(edge);
    }
}

impl ShortestPath for Dgraph {
    fn disjktra_shortest_path(&self, src: usize, dst: usize) -> isize {
        fn vertex_id_with_min_dist(dist: &[isize], visited: &[bool]) -> Option<usize> {
            let mut min_dist = isize::MAX;
            let mut vertex_id_min_dist = None;
            for (vertex_id, vertex_dist) in dist.iter().enumerate() {
                if *vertex_dist < min_dist && !visited[vertex_id] {
                    min_dist = *vertex_dist;
                    vertex_id_min_dist = Some(vertex_id);
                }
            }
            vertex_id_min_dist
        }

        let nb_vertices = self.vertices.len();
        let mut dist = (0..nb_vertices).map(|_| isize::MAX).collect::<Vec<isize>>();
        let mut prev = (0..self.vertices.len()).map(|_| 0).collect::<Vec<usize>>();
        let mut visited = (0..nb_vertices).map(|_| false).collect::<Vec<bool>>();

        dist[src] = 0;

        loop {
            let cur_vertex_id = vertex_id_with_min_dist(&dist, &visited);
            if cur_vertex_id.is_none() {
                break;
            }
            let cur_vertex_id = cur_vertex_id.unwrap();
            visited[cur_vertex_id] = true;
            for neighbor in self.vertices[cur_vertex_id].neighbors.iter() {
                let new_dist = dist[cur_vertex_id] + neighbor.weight;
                if new_dist < dist[neighbor.dst] {
                    dist[neighbor.dst] = new_dist;
                    prev[neighbor.dst] = cur_vertex_id;
                }
            }
        }
        dist[dst]
    }
}

impl fmt::Display for Dgraph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for vertex in self.vertices.iter() {
            writeln!(f, "-----------------------------------")?;
            writeln!(f, "Vertex id : {}", vertex.id)?;

            for neighbor in vertex.neighbors.iter() {
                writeln!(
                    f,
                    "Neighbor id: {} weight: {}",
                    neighbor.dst, neighbor.weight
                )?;
            }
        }
        write!(f, "")
    }
}
