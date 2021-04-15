use std::collections::{HashMap, HashSet};

use crate::geometry::shapes::{Pnt2, Seg2};

#[derive(Clone, Debug)]
pub struct PolygonData {
    edges: HashSet<Seg2>,
    vertices: Vertices,

    outgoing_edges: HashMap<Pnt2, u32>,
    incoming_edges: HashMap<Pnt2, u32>,
}

impl PolygonData {
    pub fn new() -> Self {
        Self {
            edges: HashSet::new(),
            vertices: Vertices::new(),

            outgoing_edges: HashMap::new(),
            incoming_edges: HashMap::new(),
        }
    }

    pub fn edges(&self) -> &HashSet<Seg2> {
        &self.edges
    }

    pub fn is_empty(&self) -> bool {
        self.edges.is_empty()
    }

    pub fn contains_vertex(&self, vertex: &Pnt2) -> bool {
        self.vertices.0.contains_key(vertex)
    }

    pub fn outgoing_edges(&self, vertex: &Pnt2) -> Option<u32> {
        self.outgoing_edges.get(vertex).copied()
    }

    pub fn incoming_edges(&self, vertex: &Pnt2) -> Option<u32> {
        self.incoming_edges.get(vertex).copied()
    }

    pub fn insert_edge(&mut self, edge: Seg2) {
        self.edges.insert(edge);

        self.vertices.up(edge.a);
        self.vertices.up(edge.b);

        self.incoming_edges.entry(edge.a).or_insert(0);
        self.outgoing_edges.entry(edge.b).or_insert(0);
        *self.outgoing_edges.entry(edge.a).or_insert(0) += 1;
        *self.incoming_edges.entry(edge.b).or_insert(0) += 1;
    }

    pub fn retain_edges(&mut self, mut f: impl FnMut(&Seg2) -> bool) {
        let edges = &mut self.edges;
        let vertices = &mut self.vertices;
        let outgoing_edges = &mut self.outgoing_edges;
        let incoming_edges = &mut self.incoming_edges;

        edges.retain(|edge| {
            let retain = f(edge);

            if !retain {
                let removed_a = vertices.down(edge.a);
                let removed_b = vertices.down(edge.b);

                *outgoing_edges.get_mut(&edge.a).unwrap() -= 1;
                *incoming_edges.get_mut(&edge.b).unwrap() -= 1;

                if removed_a {
                    incoming_edges.remove(&edge.a);
                    outgoing_edges.remove(&edge.a);
                }
                if removed_b {
                    incoming_edges.remove(&edge.b);
                    outgoing_edges.remove(&edge.b);
                }
            }

            retain
        });
    }
}

#[derive(Clone, Debug)]
struct Vertices(HashMap<Pnt2, u32>);

impl Vertices {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
    pub fn up(&mut self, vertex: Pnt2) {
        *self.0.entry(vertex).or_insert(0) += 1;
    }

    pub fn down(&mut self, vertex: Pnt2) -> bool {
        *self.0.get_mut(&vertex).unwrap() -= 1;

        if *self.0.get(&vertex).unwrap() == 0 {
            self.0.remove(&vertex);
            return true;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::shapes::{Pnt2, Seg2};

    use super::PolygonData;

    #[test]
    fn insert_edge_should_update_vertices() {
        let mut data = PolygonData::new();

        let a = Pnt2::new(0.0, 0.0);
        let b = Pnt2::new(1.0, 0.0);

        assert_eq!(data.contains_vertex(&a), false);
        assert_eq!(data.contains_vertex(&b), false);

        data.insert_edge(Seg2::new(a, b));

        assert_eq!(data.contains_vertex(&a), true);
        assert_eq!(data.contains_vertex(&b), true);
    }

    #[test]
    fn insert_edge_should_update_edge_counts() {
        let mut data = PolygonData::new();

        let a = Pnt2::new(0.0, 0.0);
        let b = Pnt2::new(1.0, 0.0);

        data.insert_edge(Seg2::new(a, b));

        assert_eq!(data.outgoing_edges(&a).unwrap(), 1);
        assert_eq!(data.outgoing_edges(&b).unwrap(), 0);
        assert_eq!(data.incoming_edges(&a).unwrap(), 0);
        assert_eq!(data.incoming_edges(&b).unwrap(), 1);

        data.insert_edge(Seg2::new(b, a));

        assert_eq!(data.outgoing_edges(&a).unwrap(), 1);
        assert_eq!(data.outgoing_edges(&b).unwrap(), 1);
        assert_eq!(data.incoming_edges(&a).unwrap(), 1);
        assert_eq!(data.incoming_edges(&b).unwrap(), 1);
    }

    #[test]
    fn retain_edges_should_update_vertices() {
        let mut data = PolygonData::new();

        let a = Pnt2::new(0.0, 0.0);
        let b = Pnt2::new(1.0, 0.0);
        let c = Pnt2::new(0.0, 1.0);

        let ab = Seg2::new(a, b);
        let bc = Seg2::new(b, c);

        data.insert_edge(ab);
        data.insert_edge(bc);

        data.retain_edges(|&edge| edge == ab);

        assert_eq!(data.contains_vertex(&a), true);
        assert_eq!(data.contains_vertex(&b), true);
        assert_eq!(data.contains_vertex(&c), false);
    }

    #[test]
    fn retain_edges_should_update_edge_counts() {
        let mut data = PolygonData::new();

        let a = Pnt2::new(0.0, 0.0);
        let b = Pnt2::new(1.0, 0.0);

        let ab = Seg2::new(a, b);
        let ba = Seg2::new(b, a);

        data.insert_edge(ab);
        data.insert_edge(ba);

        // Keep a -> b
        data.retain_edges(|&edge| edge == ab);

        assert_eq!(data.outgoing_edges(&a).unwrap(), 1);
        assert_eq!(data.outgoing_edges(&b).unwrap(), 0);
        assert_eq!(data.incoming_edges(&a).unwrap(), 0);
        assert_eq!(data.incoming_edges(&b).unwrap(), 1);

        // Remote last remaining edge
        data.retain_edges(|_| false);

        assert_eq!(data.outgoing_edges(&a), None);
        assert_eq!(data.outgoing_edges(&b), None);
        assert_eq!(data.incoming_edges(&a), None);
        assert_eq!(data.incoming_edges(&b), None);
    }
}
