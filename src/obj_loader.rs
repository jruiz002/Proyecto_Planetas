use raylib::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Debug)]
pub struct Vertex {
    pub position: Vector3,
    pub normal: Vector3,
}

#[derive(Clone, Debug)]
pub struct Face {
    pub vertices: Vec<usize>, // Indices de vértices
}

pub struct ObjModel {
    pub vertices: Vec<Vector3>,
    pub normals: Vec<Vector3>,
    pub faces: Vec<Face>,
}

impl ObjModel {
    pub fn load(path: &str) -> Result<Self, String> {
        let file = File::open(path).map_err(|e| format!("Error opening file: {}", e))?;
        let reader = BufReader::new(file);

        let mut vertices = Vec::new();
        let mut normals = Vec::new();
        let mut faces = Vec::new();

        for line in reader.lines() {
            let line = line.map_err(|e| format!("Error reading line: {}", e))?;
            let line = line.trim();

            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }

            match parts[0] {
                "v" => {
                    // Vértice
                    if parts.len() >= 4 {
                        let x = parts[1].parse::<f32>().unwrap_or(0.0);
                        let y = parts[2].parse::<f32>().unwrap_or(0.0);
                        let z = parts[3].parse::<f32>().unwrap_or(0.0);
                        vertices.push(Vector3::new(x, y, z));
                    }
                }
                "vn" => {
                    // Normal
                    if parts.len() >= 4 {
                        let x = parts[1].parse::<f32>().unwrap_or(0.0);
                        let y = parts[2].parse::<f32>().unwrap_or(0.0);
                        let z = parts[3].parse::<f32>().unwrap_or(0.0);
                        normals.push(Vector3::new(x, y, z));
                    }
                }
                "f" => {
                    // Cara (face)
                    let mut vertex_indices = Vec::new();
                    for i in 1..parts.len() {
                        // Formato puede ser v, v/vt, v/vt/vn, o v//vn
                        let face_part = parts[i];
                        let indices: Vec<&str> = face_part.split('/').collect();
                        
                        if let Ok(v_idx) = indices[0].parse::<usize>() {
                            // OBJ usa índices desde 1, convertimos a índices desde 0
                            vertex_indices.push(v_idx - 1);
                        }
                    }
                    
                    if vertex_indices.len() >= 3 {
                        faces.push(Face {
                            vertices: vertex_indices,
                        });
                    }
                }
                _ => {}
            }
        }

        // Si no hay normales en el archivo, las calculamos
        if normals.is_empty() {
            normals = Self::calculate_normals(&vertices, &faces);
        }

        Ok(ObjModel {
            vertices,
            normals,
            faces,
        })
    }

    fn calculate_normals(vertices: &[Vector3], faces: &[Face]) -> Vec<Vector3> {
        let mut normals = vec![Vector3::zero(); vertices.len()];
        
        // Calcular normales por cara y acumularlas en los vértices
        for face in faces {
            if face.vertices.len() >= 3 {
                let v0 = vertices[face.vertices[0]];
                let v1 = vertices[face.vertices[1]];
                let v2 = vertices[face.vertices[2]];
                
                // Calcular vectores de los lados
                let edge1 = Vector3::new(v1.x - v0.x, v1.y - v0.y, v1.z - v0.z);
                let edge2 = Vector3::new(v2.x - v0.x, v2.y - v0.y, v2.z - v0.z);
                
                // Producto cruz para obtener la normal
                let normal = Vector3::new(
                    edge1.y * edge2.z - edge1.z * edge2.y,
                    edge1.z * edge2.x - edge1.x * edge2.z,
                    edge1.x * edge2.y - edge1.y * edge2.x,
                );
                
                // Acumular en todos los vértices de la cara
                for &v_idx in &face.vertices {
                    normals[v_idx].x += normal.x;
                    normals[v_idx].y += normal.y;
                    normals[v_idx].z += normal.z;
                }
            }
        }
        
        // Normalizar
        for normal in &mut normals {
            let length = (normal.x * normal.x + normal.y * normal.y + normal.z * normal.z).sqrt();
            if length > 0.0 {
                normal.x /= length;
                normal.y /= length;
                normal.z /= length;
            }
        }
        
        normals
    }

    pub fn get_triangles(&self) -> Vec<[Vector3; 3]> {
        let mut triangles = Vec::new();
        
        for face in &self.faces {
            // Triangular la cara si tiene más de 3 vértices
            for i in 1..face.vertices.len() - 1 {
                let v0 = self.vertices[face.vertices[0]];
                let v1 = self.vertices[face.vertices[i]];
                let v2 = self.vertices[face.vertices[i + 1]];
                triangles.push([v0, v1, v2]);
            }
        }
        
        triangles
    }
}
