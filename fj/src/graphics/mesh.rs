use std::{collections::HashMap, convert::TryInto};

use euclid::default::Point3D;

use decorum::R32;

use super::{Index, Vertex};

pub struct Mesh {
    positions: Vec<[R32; 3]>,
    indices_by_vertex: HashMap<Vertex, Index>,

    vertices: Vec<Vertex>,
    indices: Vec<Index>,
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            positions: Vec::new(),
            indices_by_vertex: HashMap::new(),

            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }

    pub fn vertex(&mut self, position: [f32; 3]) -> usize {
        let i = self.positions.len();
        self.positions.push([
            R32::from_inner(position[0]),
            R32::from_inner(position[1]),
            R32::from_inner(position[2]),
        ]);
        i
    }

    pub fn triangle(&mut self, i0: usize, i1: usize, i2: usize) {
        let p0 = self.positions[i0];
        let p1 = self.positions[i1];
        let p2 = self.positions[i2];

        let normal = (Point3D::from(p1) - Point3D::from(p0))
            .cross(Point3D::from(p2) - Point3D::from(p0))
            .to_array();

        let v0 = Vertex {
            position: p0,
            normal,
        };
        let v1 = Vertex {
            position: p1,
            normal,
        };
        let v2 = Vertex {
            position: p2,
            normal,
        };

        let i0 = self.index_for_vertex(v0);
        let i1 = self.index_for_vertex(v1);
        let i2 = self.index_for_vertex(v2);

        self.indices.push(i0);
        self.indices.push(i1);
        self.indices.push(i2);
    }

    pub fn vertices(&self) -> &[Vertex] {
        self.vertices.as_slice()
    }

    pub fn indices(&self) -> &[Index] {
        self.indices.as_slice()
    }

    fn index_for_vertex(&mut self, vertex: Vertex) -> Index {
        let vertices = &mut self.vertices;

        let index = self.indices_by_vertex.entry(vertex).or_insert_with(|| {
            let index = vertices.len();
            vertices.push(vertex);
            index.try_into().unwrap()
        });

        *index
    }
}
