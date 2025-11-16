//! Vertex Shader - Transforma vértices del espacio del modelo al espacio de pantalla
//! 
//! Este módulo implementa la primera etapa del pipeline gráfico.
//! Aplica las 4 matrices de transformación en orden:
//! 1. Model Matrix (espacio modelo → mundo)
//! 2. View Matrix (espacio mundo → cámara)
//! 3. Projection Matrix (espacio cámara → clip/perspectiva)
//! 4. Viewport Matrix (espacio clip → pantalla)

use raylib::prelude::*;
use crate::matrix::Matrix;

/// Representa un vértice transformado por el vertex shader
#[derive(Clone, Copy, Debug)]
pub struct VertexShaderOutput {
    /// Posición final en screen space (después de viewport transform)
    pub screen_position: Vector3,
    /// Normal transformada al world space (para iluminación)
    pub world_normal: Vector3,
    /// Color del vértice
    pub color: Color,
}

/// Vertex Shader - Transforma un vértice usando las 4 matrices
///
/// # Pipeline de Transformación:
/// ```text
/// Vertex (modelo) 
///   → Model Matrix (escala + traslación)
///   → View Matrix (cámara)
///   → Projection Matrix (perspectiva)
///   → Viewport Matrix (pantalla)
///   → Screen Position
/// ```
pub fn vertex_shader(
    vertex_position: Vector3,
    vertex_normal: Vector3,
    vertex_color: Color,
    model_matrix: &Matrix,
    view_matrix: &Matrix,
    projection_matrix: &Matrix,
    viewport_matrix: &Matrix,
) -> VertexShaderOutput {
    
    // PASO 1: Model Space → World Space
    // Aplicar escala y traslación del objeto
    let world_position = model_matrix.transform_point(vertex_position);
    
    // Transformar normal (solo rotación, no traslación)
    let world_normal = transform_normal(vertex_normal, model_matrix);
    
    // PASO 2: World Space → View Space (Camera Space)
    // Transformar al espacio de la cámara
    let view_position = view_matrix.transform_point(world_position);
    
    // PASO 3: View Space → Clip Space (Perspective)
    // Aplicar proyección perspectiva
    let clip_position = projection_matrix.transform_point(view_position);
    
    // PASO 4: Perspective Divide (Clip Space → NDC)
    // Dividir por W para obtener coordenadas normalizadas [-1, 1]
    let w = clip_position.z.max(0.001); // Evitar división por 0
    let ndc_position = Vector3::new(
        clip_position.x / w,
        clip_position.y / w,
        clip_position.z / w,
    );
    
    // PASO 5: NDC → Screen Space
    // Transformar a coordenadas de pantalla [0, width] x [0, height]
    let screen_position = viewport_matrix.transform_point(ndc_position);
    
    VertexShaderOutput {
        screen_position,
        world_normal,
        color: vertex_color,
    }
}

/// Transforma una normal usando solo la rotación de la matriz modelo
/// (Las normales no se trasladan, solo rotan)
fn transform_normal(normal: Vector3, model_matrix: &Matrix) -> Vector3 {
    // Extraer solo la parte de rotación de la matriz (3x3 superior-izquierda)
    let transformed = Vector3::new(
        model_matrix.m[0][0] * normal.x + model_matrix.m[0][1] * normal.y + model_matrix.m[0][2] * normal.z,
        model_matrix.m[1][0] * normal.x + model_matrix.m[1][1] * normal.y + model_matrix.m[1][2] * normal.z,
        model_matrix.m[2][0] * normal.x + model_matrix.m[2][1] * normal.y + model_matrix.m[2][2] * normal.z,
    );
    
    // Normalizar
    let length = (transformed.x * transformed.x + 
                  transformed.y * transformed.y + 
                  transformed.z * transformed.z).sqrt();
    
    if length > 0.0 {
        Vector3::new(
            transformed.x / length,
            transformed.y / length,
            transformed.z / length,
        )
    } else {
        Vector3::new(0.0, 1.0, 0.0) // Normal por defecto
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matrix::Matrix;
    
    #[test]
    fn test_vertex_shader_identity_matrices() {
        let identity = Matrix::identity();
        let result = vertex_shader(
            Vector3::new(1.0, 1.0, 1.0),
            Vector3::new(0.0, 1.0, 0.0),
            Color::WHITE,
            &identity,
            &identity,
            &identity,
            &identity,
        );
        
        // Con matrices identidad, la posición debería mantenerse
        assert_eq!(result.screen_position.x, 1.0);
        assert_eq!(result.screen_position.y, 1.0);
    }
}
