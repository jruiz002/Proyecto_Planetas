//! Primitive Assembly - Ensambla vértices en primitivas geométricas (triángulos)
//! 
//! Esta es la segunda etapa del pipeline gráfico.
//! Toma vértices procesados por el vertex shader y los agrupa en triángulos.

use raylib::prelude::*;
use crate::vertex_shader::VertexShaderOutput;

/// Representa un triángulo completo listo para rasterizar
#[derive(Clone, Copy, Debug)]
pub struct Triangle {
    /// Los 3 vértices que forman el triángulo
    pub vertices: [VertexShaderOutput; 3],
}

impl Triangle {
    /// Crea un nuevo triángulo desde 3 vértices procesados
    pub fn new(v0: VertexShaderOutput, v1: VertexShaderOutput, v2: VertexShaderOutput) -> Self {
        Triangle {
            vertices: [v0, v1, v2],
        }
    }
    
    /// Calcula el centroide del triángulo (para depth sorting)
    pub fn centroid_z(&self) -> f32 {
        (self.vertices[0].screen_position.z +
         self.vertices[1].screen_position.z +
         self.vertices[2].screen_position.z) / 3.0
    }
    
    /// Verifica si el triángulo es visible (backface culling)
    /// Usa el producto cruzado en 2D para determinar la orientación
    pub fn is_front_facing(&self) -> bool {
        let v0 = &self.vertices[0].screen_position;
        let v1 = &self.vertices[1].screen_position;
        let v2 = &self.vertices[2].screen_position;
        
        // Calcular vectores de los bordes
        let edge1_x = v1.x - v0.x;
        let edge1_y = v1.y - v0.y;
        let edge2_x = v2.x - v0.x;
        let edge2_y = v2.y - v0.y;
        
        // Producto cruzado en 2D (solo componente Z)
        let cross_z = edge1_x * edge2_y - edge1_y * edge2_x;
        
        // Si cross_z > 0, el triángulo está orientado hacia adelante (CCW)
        cross_z > 0.0
    }
    
    /// Verifica si al menos un vértice está dentro de los límites de la pantalla
    pub fn is_on_screen(&self, screen_width: f32, screen_height: f32) -> bool {
        self.vertices.iter().any(|v| {
            v.screen_position.x >= -10.0 && v.screen_position.x < screen_width + 10.0 &&
            v.screen_position.y >= -10.0 && v.screen_position.y < screen_height + 10.0
        })
    }
}

/// Ensambla una lista de vértices en triángulos
/// 
/// # Argumentos
/// * `vertices` - Lista de vértices ya transformados por el vertex shader
/// * `indices` - Índices que definen cómo conectar los vértices en triángulos
///               (cada 3 índices forman un triángulo)
/// 
/// # Returns
/// Vector de triángulos ensamblados
pub fn assemble_triangles(
    vertices: &[VertexShaderOutput],
    indices: &[usize],
) -> Vec<Triangle> {
    let mut triangles = Vec::new();
    
    // Procesar índices de 3 en 3 para formar triángulos
    for chunk in indices.chunks(3) {
        if chunk.len() == 3 {
            let v0 = vertices[chunk[0]];
            let v1 = vertices[chunk[1]];
            let v2 = vertices[chunk[2]];
            
            triangles.push(Triangle::new(v0, v1, v2));
        }
    }
    
    triangles
}

/// Ensambla triángulos desde una cara con N vértices (triangulación en abanico)
/// 
/// Convierte polígonos con más de 3 vértices en múltiples triángulos.
/// Usa el primer vértice como pivote y conecta con los demás.
/// 
/// Ejemplo: Cuadrado (v0, v1, v2, v3) → Triángulos (v0,v1,v2) y (v0,v2,v3)
pub fn assemble_fan_triangles(vertices: &[VertexShaderOutput]) -> Vec<Triangle> {
    let mut triangles = Vec::new();
    
    if vertices.len() < 3 {
        return triangles;
    }
    
    let v0 = vertices[0];
    
    // Crear triángulos en abanico desde el primer vértice
    for i in 1..(vertices.len() - 1) {
        let v1 = vertices[i];
        let v2 = vertices[i + 1];
        
        triangles.push(Triangle::new(v0, v1, v2));
    }
    
    triangles
}

/// Filtra triángulos aplicando backface culling y frustum culling
pub fn cull_triangles(
    triangles: Vec<Triangle>,
    screen_width: f32,
    screen_height: f32,
    enable_backface_culling: bool,
) -> Vec<Triangle> {
    triangles.into_iter()
        .filter(|tri| {
            // Frustum culling: verificar que esté en pantalla
            if !tri.is_on_screen(screen_width, screen_height) {
                return false;
            }
            
            // Backface culling: eliminar caras traseras
            if enable_backface_culling && !tri.is_front_facing() {
                return false;
            }
            
            true
        })
        .collect()
}

/// Ordena triángulos por profundidad (painter's algorithm)
/// Los triángulos más lejanos se dibujan primero
pub fn sort_triangles_by_depth(mut triangles: Vec<Triangle>) -> Vec<Triangle> {
    triangles.sort_by(|a, b| {
        // Ordenar de mayor a menor profundidad (más lejos primero)
        b.centroid_z().partial_cmp(&a.centroid_z()).unwrap_or(std::cmp::Ordering::Equal)
    });
    triangles
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_triangle_backface_culling() {
        // Crear triángulo CCW (frente)
        let v0 = VertexShaderOutput {
            screen_position: Vector3::new(0.0, 0.0, 1.0),
            world_normal: Vector3::new(0.0, 0.0, 1.0),
            color: Color::WHITE,
        };
        let v1 = VertexShaderOutput {
            screen_position: Vector3::new(1.0, 0.0, 1.0),
            world_normal: Vector3::new(0.0, 0.0, 1.0),
            color: Color::WHITE,
        };
        let v2 = VertexShaderOutput {
            screen_position: Vector3::new(0.0, 1.0, 1.0),
            world_normal: Vector3::new(0.0, 0.0, 1.0),
            color: Color::WHITE,
        };
        
        let tri = Triangle::new(v0, v1, v2);
        assert!(tri.is_front_facing());
    }
    
    #[test]
    fn test_fan_triangulation() {
        let vertices = vec![
            VertexShaderOutput {
                screen_position: Vector3::new(0.0, 0.0, 1.0),
                world_normal: Vector3::new(0.0, 0.0, 1.0),
                color: Color::WHITE,
            },
            VertexShaderOutput {
                screen_position: Vector3::new(1.0, 0.0, 1.0),
                world_normal: Vector3::new(0.0, 0.0, 1.0),
                color: Color::WHITE,
            },
            VertexShaderOutput {
                screen_position: Vector3::new(1.0, 1.0, 1.0),
                world_normal: Vector3::new(0.0, 0.0, 1.0),
                color: Color::WHITE,
            },
            VertexShaderOutput {
                screen_position: Vector3::new(0.0, 1.0, 1.0),
                world_normal: Vector3::new(0.0, 0.0, 1.0),
                color: Color::WHITE,
            },
        ];
        
        let triangles = assemble_fan_triangles(&vertices);
        assert_eq!(triangles.len(), 2); // Cuadrado = 2 triángulos
    }
}
