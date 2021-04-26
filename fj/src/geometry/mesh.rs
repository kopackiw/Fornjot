use std::{collections::HashMap, convert::TryInto};

use decorum::R32;
use nalgebra::Vector3;

use crate::{geometry::shapes::Point, graphics};

pub struct Mesh {
    indices_by_vertex: HashMap<Vertex, graphics::Index>,

    vertices: Vec<Vertex>,
    indices: Vec<graphics::Index>,
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            indices_by_vertex: HashMap::new(),

            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }

    pub fn triangle(
        &mut self,
        p0: impl Into<Point<3>>,
        p1: impl Into<Point<3>>,
        p2: impl Into<Point<3>>,
    ) {
        let p0: nalgebra::Point<f32, 3> = p0.into().into();
        let p1: nalgebra::Point<f32, 3> = p1.into().into();
        let p2: nalgebra::Point<f32, 3> = p2.into().into();

        let normal = (p1 - p0).cross(&(p2 - p0)).normalize();
        let normal = normal.map(|v| R32::from_inner(v));

        let v0 = Vertex {
            position: p0.into(),
            normal,
        };
        let v1 = Vertex {
            position: p1.into(),
            normal,
        };
        let v2 = Vertex {
            position: p2.into(),
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

    pub fn indices(&self) -> &[graphics::Index] {
        self.indices.as_slice()
    }

    pub fn into_graphics_mesh(self) -> graphics::Mesh {
        let vertices = self
            .vertices
            .into_iter()
            .map(|vertex| graphics::Vertex {
                position: [
                    vertex.position[0].into_inner(),
                    vertex.position[1].into_inner(),
                    vertex.position[2].into_inner(),
                ],
                normal: [
                    vertex.normal[0].into_inner(),
                    vertex.normal[1].into_inner(),
                    vertex.normal[2].into_inner(),
                ],
            })
            .collect();
        let indices = self.indices;

        graphics::Mesh { vertices, indices }
    }

    fn index_for_vertex(&mut self, vertex: Vertex) -> graphics::Index {
        let vertices = &mut self.vertices;

        let index = self.indices_by_vertex.entry(vertex).or_insert_with(|| {
            let index = vertices.len();
            vertices.push(vertex);
            index.try_into().unwrap()
        });

        *index
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Vertex {
    pub position: Point<3>,
    pub normal: Vector3<R32>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Index(u16);

#[cfg(test)]
mod tests {
    use decorum::R32;
    use nalgebra::{Point3, Vector3};

    use super::{Mesh, Vertex};

    #[test]
    fn mesh_should_convert_triangle_into_vertices_and_indices() {
        let mut mesh = Mesh::new();

        let v0 = [0.0, 0.0, 0.0];
        let v1 = [0.5, 0.0, 0.0];
        let v2 = [0.0, 0.5, 0.0];

        mesh.triangle(v0, v1, v2);

        let mut vertices = Vec::new();
        for &i in mesh.indices() {
            vertices.push(mesh.vertices()[i as usize]);
        }

        let normal = Vector3::new(
            R32::from_inner(0.0),
            R32::from_inner(0.0),
            R32::from_inner(1.0),
        );
        assert_eq!(
            vertices,
            vec![
                Vertex {
                    position: Point3::from(
                        Point3::from(v0).coords.map(|f| R32::from_inner(f))
                    )
                    .into(),
                    normal,
                },
                Vertex {
                    position: Point3::from(
                        Point3::from(v1).coords.map(|f| R32::from_inner(f))
                    )
                    .into(),
                    normal,
                },
                Vertex {
                    position: Point3::from(
                        Point3::from(v2).coords.map(|f| R32::from_inner(f))
                    )
                    .into(),
                    normal,
                },
            ]
        );
    }

    // TASK: Add method that inverts triangles of a mesh.
    // TASK: Add method that merges another mesh into the mesh.
}
