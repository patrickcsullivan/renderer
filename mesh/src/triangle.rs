use super::Mesh;
use cgmath::{Point2, Point3};

/// A reference to an individual triangle in a mesh.
#[derive(Debug, Clone, Copy)]
pub struct Triangle<'msh> {
    mesh: &'msh Mesh,
    index_in_mesh: usize,
}

impl<'msh> Triangle<'msh> {
    /// Returns the positions of the triangle's vertices in world space.
    pub fn world_space_vertices(&self) -> (Point3<f32>, Point3<f32>, Point3<f32>) {
        let (i1, i2, i3) = self.mesh.triangle_vertex_indices[self.index_in_mesh];
        let p1 = self.mesh.world_space_vertices[i1];
        let p2 = self.mesh.world_space_vertices[i2];
        let p3 = self.mesh.world_space_vertices[i3];
        (p1, p2, p3)
    }

    /// Returns the UV coordinates for each of the triangle's vertices. If the
    /// mesh does not contain UV coordinates, then default coordinates are
    /// returned.
    pub fn uv_vertices(&self) -> (Point2<f32>, Point2<f32>, Point2<f32>) {
        if let Some(uvs) = &self.mesh.uvs {
            let (i1, i2, i3) = self.mesh.triangle_vertex_indices[self.index_in_mesh];
            (uvs[i1], uvs[i2], uvs[i3])
        } else {
            (
                Point2::new(0.0, 0.0),
                Point2::new(1.0, 0.0),
                Point2::new(1.0, 1.0),
            )
        }
    }
}

impl<'msh> Mesh {
    pub fn triangle_at(&'msh self, index: usize) -> Triangle<'msh> {
        Triangle {
            mesh: self,
            index_in_mesh: index,
        }
    }

    pub fn triangles(&'msh self) -> Vec<Triangle<'msh>> {
        let triangle_count = self.triangle_vertex_indices.len();
        (0..triangle_count).map(|i| self.triangle_at(i)).collect()
    }
}
