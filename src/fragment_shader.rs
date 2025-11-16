//! Fragment Shader - Calcula el color final de cada fragmento
//! 
//! Esta es la cuarta y última etapa del pipeline gráfico.
//! Toma fragmentos del rasterizador y aplica iluminación y efectos
//! para calcular el color final que se escribirá en el framebuffer.

use raylib::prelude::*;

/// Configuración de iluminación para el fragment shader
pub struct LightingConfig {
    /// Dirección de la luz principal (normalizada)
    pub light_direction: Vector3,
    /// Intensidad de luz ambiental [0.0, 1.0]
    pub ambient_intensity: f32,
    /// Intensidad de luz difusa [0.0, 1.0]
    pub diffuse_intensity: f32,
    /// Habilitar iluminación (si false, solo usa color base)
    pub enable_lighting: bool,
}

impl Default for LightingConfig {
    fn default() -> Self {
        LightingConfig {
            light_direction: Vector3::new(-1.0, -1.0, -1.0).normalized(),
            ambient_intensity: 0.3,
            diffuse_intensity: 0.7,
            enable_lighting: true,
        }
    }
}

/// Fragment Shader - Calcula el color final aplicando iluminación
/// 
/// # Modelo de Iluminación (Phong simplificado):
/// ```text
/// Color Final = Color Base * (Ambiente + Difuso)
/// 
/// Ambiente = ambient_intensity
/// Difuso = diffuse_intensity * max(0, dot(normal, light_direction))
/// ```
/// 
/// # Argumentos
/// * `base_color` - Color base del fragmento (interpolado del vértice)
/// * `normal` - Normal interpolada del fragmento (debe estar normalizada)
/// * `config` - Configuración de iluminación
/// 
/// # Returns
/// Color final con iluminación aplicada
pub fn fragment_shader(
    base_color: Color,
    normal: Vector3,
    config: &LightingConfig,
) -> Color {
    // Si la iluminación está deshabilitada, retornar color base
    if !config.enable_lighting {
        return base_color;
    }
    
    // Normalizar la normal (por si acaso la interpolación la desnormalizó)
    let normalized_normal = normalize_vector3(normal);
    
    // Calcular componente ambiental
    let ambient = config.ambient_intensity;
    
    // Calcular componente difusa (Lambertian)
    // dot(N, L) = cuánto apunta la superficie hacia la luz
    let light_dir = normalize_vector3(config.light_direction);
    let diffuse_factor = dot_product(normalized_normal, light_dir).max(0.0);
    let diffuse = config.diffuse_intensity * diffuse_factor;
    
    // Iluminación total
    let total_light = (ambient + diffuse).min(1.0);
    
    // Aplicar iluminación al color base
    apply_lighting_to_color(base_color, total_light)
}

/// Fragment Shader para objetos emisivos (sin iluminación)
/// 
/// Usado para objetos que emiten luz (como el sol o estrellas)
/// No aplica ningún cálculo de iluminación, solo retorna el color base
pub fn fragment_shader_emissive(base_color: Color) -> Color {
    base_color
}

/// Fragment Shader con iluminación Phong completa (ambiente + difusa + especular)
/// 
/// # Argumentos
/// * `base_color` - Color base del fragmento
/// * `normal` - Normal del fragmento (normalizada)
/// * `view_direction` - Dirección hacia la cámara (normalizada)
/// * `light_direction` - Dirección hacia la luz (normalizada)
/// * `ambient` - Intensidad ambiental
/// * `diffuse` - Intensidad difusa
/// * `specular` - Intensidad especular
/// * `shininess` - Exponente especular (brillo)
pub fn fragment_shader_phong(
    base_color: Color,
    normal: Vector3,
    view_direction: Vector3,
    light_direction: Vector3,
    ambient: f32,
    diffuse: f32,
    specular: f32,
    shininess: f32,
) -> Color {
    let n = normalize_vector3(normal);
    let l = normalize_vector3(light_direction);
    let v = normalize_vector3(view_direction);
    
    // Componente ambiental
    let ambient_component = ambient;
    
    // Componente difusa
    let n_dot_l = dot_product(n, l).max(0.0);
    let diffuse_component = diffuse * n_dot_l;
    
    // Componente especular (Blinn-Phong)
    let h = normalize_vector3(Vector3::new(
        l.x + v.x,
        l.y + v.y,
        l.z + v.z,
    ));
    let n_dot_h = dot_product(n, h).max(0.0);
    let specular_component = if n_dot_l > 0.0 {
        specular * n_dot_h.powf(shininess)
    } else {
        0.0
    };
    
    let total_light = (ambient_component + diffuse_component + specular_component).min(1.0);
    
    apply_lighting_to_color(base_color, total_light)
}

/// Aplica un factor de iluminación a un color
#[inline]
fn apply_lighting_to_color(color: Color, light_factor: f32) -> Color {
    Color {
        r: (color.r as f32 * light_factor).clamp(0.0, 255.0) as u8,
        g: (color.g as f32 * light_factor).clamp(0.0, 255.0) as u8,
        b: (color.b as f32 * light_factor).clamp(0.0, 255.0) as u8,
        a: color.a,
    }
}

/// Normaliza un Vector3
#[inline]
fn normalize_vector3(v: Vector3) -> Vector3 {
    let length = (v.x * v.x + v.y * v.y + v.z * v.z).sqrt();
    if length > 0.0 {
        Vector3::new(v.x / length, v.y / length, v.z / length)
    } else {
        Vector3::new(0.0, 1.0, 0.0) // Vector por defecto
    }
}

/// Calcula el producto punto entre dos vectores
#[inline]
fn dot_product(a: Vector3, b: Vector3) -> f32 {
    a.x * b.x + a.y * b.y + a.z * b.z
}

/// Mezcla dos colores con un factor alpha
pub fn blend_colors(color1: Color, color2: Color, alpha: f32) -> Color {
    let a = alpha.clamp(0.0, 1.0);
    let inv_a = 1.0 - a;
    
    Color {
        r: (color1.r as f32 * inv_a + color2.r as f32 * a) as u8,
        g: (color1.g as f32 * inv_a + color2.g as f32 * a) as u8,
        b: (color1.b as f32 * inv_a + color2.b as f32 * a) as u8,
        a: (color1.a as f32 * inv_a + color2.a as f32 * a) as u8,
    }
}

/// Ajusta el brillo de un color
pub fn adjust_brightness(color: Color, factor: f32) -> Color {
    Color {
        r: (color.r as f32 * factor).clamp(0.0, 255.0) as u8,
        g: (color.g as f32 * factor).clamp(0.0, 255.0) as u8,
        b: (color.b as f32 * factor).clamp(0.0, 255.0) as u8,
        a: color.a,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_emissive_shader() {
        let color = Color::RED;
        let result = fragment_shader_emissive(color);
        assert_eq!(result.r, color.r);
        assert_eq!(result.g, color.g);
        assert_eq!(result.b, color.b);
    }
    
    #[test]
    fn test_fragment_shader_no_lighting() {
        let config = LightingConfig {
            enable_lighting: false,
            ..Default::default()
        };
        
        let color = Color::BLUE;
        let normal = Vector3::new(0.0, 1.0, 0.0);
        
        let result = fragment_shader(color, normal, &config);
        assert_eq!(result.r, color.r);
        assert_eq!(result.g, color.g);
        assert_eq!(result.b, color.b);
    }
    
    #[test]
    fn test_normalize_vector() {
        let v = Vector3::new(3.0, 4.0, 0.0);
        let normalized = normalize_vector3(v);
        
        // La longitud debe ser 1.0
        let length = (normalized.x * normalized.x + 
                      normalized.y * normalized.y + 
                      normalized.z * normalized.z).sqrt();
        assert!((length - 1.0).abs() < 0.001);
    }
    
    #[test]
    fn test_dot_product() {
        let a = Vector3::new(1.0, 0.0, 0.0);
        let b = Vector3::new(1.0, 0.0, 0.0);
        assert_eq!(dot_product(a, b), 1.0);
        
        let c = Vector3::new(1.0, 0.0, 0.0);
        let d = Vector3::new(0.0, 1.0, 0.0);
        assert_eq!(dot_product(c, d), 0.0); // Perpendiculares
    }
    
    #[test]
    fn test_brightness_adjustment() {
        let color = Color::new(100, 100, 100, 255);
        let brighter = adjust_brightness(color, 2.0);
        assert_eq!(brighter.r, 200);
        assert_eq!(brighter.g, 200);
        assert_eq!(brighter.b, 200);
    }
}
