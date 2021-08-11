mod triangle;

use cgmath::{Matrix4, Point2, Point3, Transform, Vector3};
pub use triangle::Triangle;

/// A mesh of triangles.
#[derive(Debug)]
pub struct Mesh {
    /// The vertices in world space that make up the mesh.
    pub world_space_vertices: Vec<Point3<f32>>,

    /// An array that describes each triangle in the mesh. Each element of the
    /// array is a tuple that contains three indices into the `vertices` array.
    pub triangle_vertex_indices: Vec<(usize, usize, usize)>,

    /// An array containing a tangent vector for each vertex in the mesh.
    pub tangents: Option<Vec<Vector3<f32>>>,

    /// An array containing a normal vector for each vertex in the mesh.
    pub normals: Option<Vec<Vector3<f32>>>,

    /// An array containing a UV coordinate for each vertex in the mesh.
    pub uvs: Option<Vec<Point2<f32>>>,
}

pub struct MeshBuilder {
    object_to_world: Matrix4<f32>,
    object_space_vertices: Vec<Point3<f32>>,
    triangle_vertex_indices: Vec<(usize, usize, usize)>,
    tangents: Option<Vec<Vector3<f32>>>,
    normals: Option<Vec<Vector3<f32>>>,
    uvs: Option<Vec<Point2<f32>>>,
}

impl MeshBuilder {
    pub fn new(
        object_space_vertices: Vec<Point3<f32>>,
        triangle_vertex_indices: Vec<(usize, usize, usize)>,
    ) -> Self {
        Self {
            object_to_world: Matrix4::from_scale(1.0),
            object_space_vertices,
            triangle_vertex_indices,
            tangents: None,
            normals: None,
            uvs: None,
        }
    }

    pub fn object_to_world(mut self, object_to_world: Matrix4<f32>) -> Self {
        self.object_to_world = object_to_world;
        self
    }

    pub fn tangents(mut self, tangents: Vec<Vector3<f32>>) -> Self {
        self.tangents = Some(tangents);
        self
    }

    pub fn normals(mut self, normals: Vec<Vector3<f32>>) -> Self {
        self.normals = Some(normals);
        self
    }

    pub fn uvs(mut self, uvs: Vec<Point2<f32>>) -> Self {
        self.uvs = Some(uvs);
        self
    }

    pub fn build(self) -> Mesh {
        Mesh {
            world_space_vertices: self
                .object_space_vertices
                .iter()
                .map(|p| self.object_to_world.transform_point(*p))
                .collect(),
            triangle_vertex_indices: self.triangle_vertex_indices,
            tangents: self.tangents,
            normals: self.normals,
            uvs: self.uvs,
        }
    }

    pub fn from_stl<R>(stl_bytes: &mut R) -> Result<MeshBuilder, nom_stl::Error>
    where
        R: std::io::Read + std::io::Seek,
    {
        let stl = nom_stl::parse_stl(stl_bytes)?;
        let num_triangles = stl.triangles().len();

        let mut vertices = vec![Point3::new(0.0, 0.0, 0.0); num_triangles * 3];
        let mut normals = vec![Vector3::new(0.0, 0.0, 0.0); num_triangles * 3];
        let mut triangle_vertex_indices = vec![(0, 0, 0); num_triangles];

        for (i, t) in stl.triangles().iter().enumerate() {
            let [[v1x, v1y, v1z], [v2x, v2y, v2z], [v3x, v3y, v3z]] = t.vertices();
            vertices[3 * i] = Point3::new(v1x, v1y, v1z);
            vertices[(3 * i) + 1] = Point3::new(v2x, v2y, v2z);
            vertices[(3 * i) + 2] = Point3::new(v3x, v3y, v3z);

            let [nx, ny, nz] = t.normal();
            let normal = Vector3::new(nx, ny, nz);
            normals[3 * i] = normal;
            normals[(3 * i) + 1] = normal;
            normals[(3 * i) + 2] = normal;

            triangle_vertex_indices[i] = (3 * i, (3 * i) + 1, (3 * i) + 2);
        }

        Ok(MeshBuilder::new(vertices, triangle_vertex_indices).normals(normals))
    }
}
