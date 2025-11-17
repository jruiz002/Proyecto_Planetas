//! Rasterizer - Convierte triángulos en fragmentos (píxeles)
//! 
//! Esta es la tercera etapa del pipeline gráfico.
//! Toma triángulos ensamblados y los convierte en fragmentos individuales
//! usando coordenadas baricéntricas para interpolación.

use raylib::prelude::*;
use crate::primitive_assembly::Triangle;

/// Representa un fragmento (píxel candidato) generado por el rasterizador
#[derive(Clone, Copy, Debug)]
pub struct Fragment {
    /// Posición en pantalla (coordenadas de píxel)
    pub screen_x: i32,
    pub screen_y: i32,
    /// Profundidad interpolada (para Z-buffer)
    pub depth: f32,
    /// Normal interpolada del fragmento
    pub normal: Vector3,
    /// Color interpolado del fragmento
    pub color: Color,
}

/// Rasteriza un triángulo en fragmentos usando coordenadas baricéntricas
/// 
/// # Algoritmo:
/// 1. Calcula la bounding box del triángulo
/// 2. Aplica clipping a los límites de pantalla (frustum culling)
/// 3. Para cada píxel en la bounding box clipeada:
///    - Calcula coordenadas baricéntricas (u, v, w)
///    - Si está dentro del triángulo (u,v,w ≥ 0 y u+v+w=1):
///      - Interpola atributos (profundidad, normal, color)
///      - Genera un fragmento
/// 
/// # Argumentos
/// * `triangle` - Triángulo a rasterizar
/// * `screen_width` - Ancho de la pantalla (para clipping)
/// * `screen_height` - Alto de la pantalla (para clipping)
/// 
/// # Returns
/// Vector de fragmentos que cubren el triángulo (solo dentro de pantalla)
pub fn rasterize_triangle(triangle: &Triangle, screen_width: f32, screen_height: f32) -> Vec<Fragment> {
    let mut fragments = Vec::new();
    
    let v0 = &triangle.vertices[0];
    let v1 = &triangle.vertices[1];
    let v2 = &triangle.vertices[2];
    
    // Calcular bounding box del triángulo
    let min_x = v0.screen_position.x.min(v1.screen_position.x).min(v2.screen_position.x).floor() as i32;
    let max_x = v0.screen_position.x.max(v1.screen_position.x).max(v2.screen_position.x).ceil() as i32;
    let min_y = v0.screen_position.y.min(v1.screen_position.y).min(v2.screen_position.y).floor() as i32;
    let max_y = v0.screen_position.y.max(v1.screen_position.y).max(v2.screen_position.y).ceil() as i32;
    
    // ===== OPTIMIZACIÓN: Frustum Culling de Fragmentos =====
    // Clipear la bounding box a los límites de pantalla
    // Esto evita procesar millones de fragmentos fuera de pantalla cuando
    // un planeta está muy cerca de la cámara
    let screen_min_x = 0;
    let screen_max_x = screen_width as i32 - 1;
    let screen_min_y = 0;
    let screen_max_y = screen_height as i32 - 1;
    
    // Aplicar clipping: solo rasterizar dentro de [0, screen_width) x [0, screen_height)
    let clipped_min_x = min_x.max(screen_min_x);
    let clipped_max_x = max_x.min(screen_max_x);
    let clipped_min_y = min_y.max(screen_min_y);
    let clipped_max_y = max_y.min(screen_max_y);
    
    // Si la bounding box clipeada está completamente fuera de pantalla, retornar vacío
    if clipped_min_x > clipped_max_x || clipped_min_y > clipped_max_y {
        return fragments;
    }
    
    // Pre-calcular área total del triángulo (para coordenadas baricéntricas)
    let total_area = edge_function(
        v0.screen_position.x, v0.screen_position.y,
        v1.screen_position.x, v1.screen_position.y,
        v2.screen_position.x, v2.screen_position.y,
    );
    
    // Si el área es muy pequeña o negativa, el triángulo es degenerado
    if total_area.abs() < 0.001 {
        return fragments;
    }
    
    let inv_total_area = 1.0 / total_area;
    
    // Iterar sobre cada píxel en la bounding box CLIPEADA (optimización de frustum culling)
    // Antes iterábamos sobre toda la bounding box del triángulo, incluso fuera de pantalla
    // Ahora solo procesamos píxeles dentro de [0, screen_width) x [0, screen_height)
    for y in clipped_min_y..=clipped_max_y {
        for x in clipped_min_x..=clipped_max_x {
            let px = x as f32 + 0.5; // Centro del píxel
            let py = y as f32 + 0.5;
            
            // Calcular coordenadas baricéntricas (w0, w1, w2)
            let w0 = edge_function(
                v1.screen_position.x, v1.screen_position.y,
                v2.screen_position.x, v2.screen_position.y,
                px, py,
            ) * inv_total_area;
            
            let w1 = edge_function(
                v2.screen_position.x, v2.screen_position.y,
                v0.screen_position.x, v0.screen_position.y,
                px, py,
            ) * inv_total_area;
            
            let w2 = edge_function(
                v0.screen_position.x, v0.screen_position.y,
                v1.screen_position.x, v1.screen_position.y,
                px, py,
            ) * inv_total_area;
            
            // Verificar si el píxel está dentro del triángulo
            // Todas las coordenadas baricéntricas deben ser ≥ 0
            if w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0 {
                // Interpolar profundidad (Z)
                let depth = w0 * v0.screen_position.z +
                           w1 * v1.screen_position.z +
                           w2 * v2.screen_position.z;
                
                // Interpolar normal
                let normal = Vector3::new(
                    w0 * v0.world_normal.x + w1 * v1.world_normal.x + w2 * v2.world_normal.x,
                    w0 * v0.world_normal.y + w1 * v1.world_normal.y + w2 * v2.world_normal.y,
                    w0 * v0.world_normal.z + w1 * v1.world_normal.z + w2 * v2.world_normal.z,
                );
                
                // Interpolar color
                let color = interpolate_color(&v0.color, &v1.color, &v2.color, w0, w1, w2);
                
                fragments.push(Fragment {
                    screen_x: x,
                    screen_y: y,
                    depth,
                    normal,
                    color,
                });
            }
        }
    }
    
    fragments
}

/// Rasteriza un triángulo con LOD (Level of Detail) - omite píxeles para optimización
/// 
/// # Argumentos
/// * `triangle` - Triángulo a rasterizar
/// * `skip_factor` - Cuántos píxeles omitir (1=todos, 2=la mitad, 4=un cuarto, etc.)
/// * `screen_width` - Ancho de la pantalla (para clipping)
/// * `screen_height` - Alto de la pantalla (para clipping)
/// 
/// # Returns
/// Vector de fragmentos (con menor resolución si skip_factor > 1)
pub fn rasterize_triangle_lod(
    triangle: &Triangle, 
    skip_factor: i32, 
    screen_width: f32, 
    screen_height: f32
) -> Vec<Fragment> {
    if skip_factor <= 1 {
        return rasterize_triangle(triangle, screen_width, screen_height);
    }
    
    let mut fragments = Vec::new();
    
    let v0 = &triangle.vertices[0];
    let v1 = &triangle.vertices[1];
    let v2 = &triangle.vertices[2];
    
    let min_x = v0.screen_position.x.min(v1.screen_position.x).min(v2.screen_position.x).floor() as i32;
    let max_x = v0.screen_position.x.max(v1.screen_position.x).max(v2.screen_position.x).ceil() as i32;
    let min_y = v0.screen_position.y.min(v1.screen_position.y).min(v2.screen_position.y).floor() as i32;
    let max_y = v0.screen_position.y.max(v1.screen_position.y).max(v2.screen_position.y).ceil() as i32;
    
    // ===== OPTIMIZACIÓN: Frustum Culling de Fragmentos (versión LOD) =====
    let screen_min_x = 0;
    let screen_max_x = screen_width as i32 - 1;
    let screen_min_y = 0;
    let screen_max_y = screen_height as i32 - 1;
    
    let clipped_min_x = min_x.max(screen_min_x);
    let clipped_max_x = max_x.min(screen_max_x);
    let clipped_min_y = min_y.max(screen_min_y);
    let clipped_max_y = max_y.min(screen_max_y);
    
    if clipped_min_x > clipped_max_x || clipped_min_y > clipped_max_y {
        return fragments;
    }
    
    let total_area = edge_function(
        v0.screen_position.x, v0.screen_position.y,
        v1.screen_position.x, v1.screen_position.y,
        v2.screen_position.x, v2.screen_position.y,
    );
    
    if total_area.abs() < 0.001 {
        return fragments;
    }
    
    let inv_total_area = 1.0 / total_area;
    
    // Iterar con saltos (LOD) usando la bounding box CLIPEADA
    let mut y = clipped_min_y;
    while y <= clipped_max_y {
        let mut x = clipped_min_x;
        while x <= clipped_max_x {
            let px = x as f32 + 0.5;
            let py = y as f32 + 0.5;
            
            let w0 = edge_function(
                v1.screen_position.x, v1.screen_position.y,
                v2.screen_position.x, v2.screen_position.y,
                px, py,
            ) * inv_total_area;
            
            let w1 = edge_function(
                v2.screen_position.x, v2.screen_position.y,
                v0.screen_position.x, v0.screen_position.y,
                px, py,
            ) * inv_total_area;
            
            let w2 = edge_function(
                v0.screen_position.x, v0.screen_position.y,
                v1.screen_position.x, v1.screen_position.y,
                px, py,
            ) * inv_total_area;
            
            if w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0 {
                let depth = w0 * v0.screen_position.z +
                           w1 * v1.screen_position.z +
                           w2 * v2.screen_position.z;
                
                let normal = Vector3::new(
                    w0 * v0.world_normal.x + w1 * v1.world_normal.x + w2 * v2.world_normal.x,
                    w0 * v0.world_normal.y + w1 * v1.world_normal.y + w2 * v2.world_normal.y,
                    w0 * v0.world_normal.z + w1 * v1.world_normal.z + w2 * v2.world_normal.z,
                );
                
                let color = interpolate_color(&v0.color, &v1.color, &v2.color, w0, w1, w2);
                
                fragments.push(Fragment {
                    screen_x: x,
                    screen_y: y,
                    depth,
                    normal,
                    color,
                });
            }
            
            x += skip_factor;
        }
        y += skip_factor;
    }
    
    fragments
}

/// Edge Function - Calcula el producto cruzado 2D
/// 
/// Determina de qué lado de una línea está un punto.
/// Si resultado > 0: punto está a la izquierda
/// Si resultado < 0: punto está a la derecha
/// Si resultado = 0: punto está sobre la línea
/// 
/// También se usa para calcular el área del triángulo (área = edge_function / 2)
#[inline]
fn edge_function(ax: f32, ay: f32, bx: f32, by: f32, cx: f32, cy: f32) -> f32 {
    (cx - ax) * (by - ay) - (cy - ay) * (bx - ax)
}

/// Interpola colores RGB usando coordenadas baricéntricas
fn interpolate_color(c0: &Color, c1: &Color, c2: &Color, w0: f32, w1: f32, w2: f32) -> Color {
    let r = (w0 * c0.r as f32 + w1 * c1.r as f32 + w2 * c2.r as f32).clamp(0.0, 255.0) as u8;
    let g = (w0 * c0.g as f32 + w1 * c1.g as f32 + w2 * c2.g as f32).clamp(0.0, 255.0) as u8;
    let b = (w0 * c0.b as f32 + w1 * c1.b as f32 + w2 * c2.b as f32).clamp(0.0, 255.0) as u8;
    let a = (w0 * c0.a as f32 + w1 * c1.a as f32 + w2 * c2.a as f32).clamp(0.0, 255.0) as u8;
    
    Color { r, g, b, a }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_edge_function_orientation() {
        // Triángulo CCW: (0,0), (1,0), (0,1)
        // Punto (0.5, 0.5) debería estar dentro
        let result = edge_function(0.0, 0.0, 1.0, 0.0, 0.5, 0.5);
        assert!(result > 0.0); // Punto a la izquierda
    }
    
    #[test]
    fn test_color_interpolation() {
        let white = Color::WHITE;
        let black = Color::BLACK;
        let gray = Color::new(128, 128, 128, 255);
        
        // 50% blanco, 50% negro = gris
        let result = interpolate_color(&white, &black, &black, 0.5, 0.25, 0.25);
        assert!(result.r > 100 && result.r < 200); // Aproximadamente gris
    }
}
