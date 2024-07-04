use stl_io::IndexedMesh;
use threemf::{
    model::{Triangle, Triangles, Vertex, Vertices},
    Mesh,
};

/// Convert an STL mesh to a 3MF mesh.
pub(crate) fn stl_to_mesh(stl: IndexedMesh) -> Mesh {
    let vertices = Vertices {
        vertex: stl
            .vertices
            .iter()
            .map(|v| Vertex {
                x: v[0].into(),
                y: v[1].into(),
                z: v[2].into(),
            })
            .collect(),
    };
    let triangles = Triangles {
        triangle: stl
            .faces
            .iter()
            .map(|f| Triangle {
                v1: f.vertices[0],
                v2: f.vertices[1],
                v3: f.vertices[2],
            })
            .collect(),
    };

    Mesh {
        vertices,
        triangles,
    }
}
