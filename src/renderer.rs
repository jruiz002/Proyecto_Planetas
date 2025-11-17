#![allow(dead_code)]
use crate::camera::Camera;
use crate::celestial_body::CelestialBody;
use crate::matrix::{create_projection_matrix, create_viewport_matrix, create_translation_matrix, create_scale_matrix, Matrix};
use crate::solar_system::SolarSystem;
use crate::framebuffer::Framebuffer;
use crate::obj_loader::ObjModel;
use crate::vertex_shader::{vertex_shader, VertexShaderOutput};
use crate::fragment_shader::{fragment_shader, LightingConfig};
use crate::primitive_assembly::Triangle;
use crate::rasterizer::rasterize_triangle;
use rand::Rng;
use raylib::prelude::*;
use std::f32::consts::PI;

pub struct Renderer {
    pub screen_width: f32,
    pub screen_height: f32,
    pub projection_matrix: Matrix,
    pub viewport_matrix: Matrix,
    pub stars: Vec<Vector3>,
    pub show_wireframe: bool,
    pub lighting_config: LightingConfig,
    pub sphere_model: Option<ObjModel>,
}

impl Renderer {
    pub fn new(screen_width: f32, screen_height: f32) -> Self {
        let projection_matrix = create_projection_matrix(
            PI / 4.0,
            screen_width / screen_height,
            0.1,
            1000.0,
        );

        let viewport_matrix = create_viewport_matrix(0.0, 0.0, screen_width, screen_height);

        let sphere_model = match ObjModel::load("assets/sphere.obj") {
            Ok(model) => {
                println!("Sphere model loaded: {} vertices", model.vertices.len());
                Some(model)
            }
            Err(e) => {
                eprintln!("Error loading sphere.obj: {}", e);
                None
            }
        };

        let mut renderer = Renderer {
            screen_width,
            screen_height,
            projection_matrix,
            viewport_matrix,
            stars: Vec::new(),
            show_wireframe: false,
            lighting_config: LightingConfig::default(),
            sphere_model,
        };

        renderer.generate_stars(1500);
        renderer
    }

    fn generate_stars(&mut self, count: usize) {
        let mut rng = rand::thread_rng();
        self.stars.clear();

        for _ in 0..count {
            let theta = rng.gen::<f32>() * 2.0 * PI;
            let phi = rng.gen::<f32>() * PI;
            let distance = 800.0 + rng.gen::<f32>() * 200.0;

            let x = distance * phi.sin() * theta.cos();
            let y = distance * phi.cos();
            let z = distance * phi.sin() * theta.sin();

            self.stars.push(Vector3::new(x, y, z));
        }
    }

    // ===== MAIN FRAMEBUFFER RENDERING =====
    
    pub fn render_solar_system_to_framebuffer(
        &self,
        framebuffer: &mut Framebuffer,
        solar_system: &SolarSystem,
        camera: &Camera,
        show_orbits: bool,
    ) {
        let view_matrix = camera.get_view_matrix();

        // 1. Background gradient
        self.render_space_gradient_to_framebuffer(framebuffer);

        // 2. Stars
        self.render_stars_to_framebuffer(framebuffer, &view_matrix);

        // 3. Celestial bodies (sorted by distance)
        let bodies = solar_system.get_bodies_sorted_by_distance(camera.get_position());
        for body in bodies {
            self.render_celestial_body_to_framebuffer(framebuffer, body, &view_matrix);
            for moon in &body.moons {
                self.render_celestial_body_to_framebuffer(framebuffer, moon, &view_matrix);
            }
        }

        // 4. Orbits
        if show_orbits {
            self.render_orbits_to_framebuffer(framebuffer, solar_system, &view_matrix);
        }
    }

    // ===== BACKGROUND & STARS =====

    fn render_space_gradient_to_framebuffer(&self, framebuffer: &mut Framebuffer) {
        let width = framebuffer.width as f32;
        let height = framebuffer.height as f32;
        let center_x = width * 0.5;
        let center_y = height * 0.5;
        let max_dist = ((center_x * center_x) + (center_y * center_y)).sqrt();
        
        // 2x2 pixel blocks for 4x speedup
        let pixel_step = 2;
        
        for y in (0..framebuffer.height).step_by(pixel_step) {
            let dy = y as f32 - center_y;
            for x in (0..framebuffer.width).step_by(pixel_step) {
                let dx = x as f32 - center_x;
                let distance = (dx * dx + dy * dy).sqrt();
                let norm_dist = (distance / max_dist).min(1.0);
                
                let base_intensity = norm_dist * 0.8;
                
                let r = (5.0 + base_intensity * 25.0).min(30.0) as u8;
                let g = (3.0 + base_intensity * 12.0).min(15.0) as u8;
                let b = (15.0 + base_intensity * 40.0).min(55.0) as u8;
                
                let color = Color::new(r, g, b, 255);
                
                // Fill 2x2 block
                for dy_block in 0..pixel_step {
                    for dx_block in 0..pixel_step {
                        let px = x + dx_block as u32;
                        let py = y + dy_block as u32;
                        if px < framebuffer.width && py < framebuffer.height {
                            framebuffer.set_pixel_color(px, py, color);
                        }
                    }
                }
            }
        }
    }

    fn render_stars_to_framebuffer(&self, framebuffer: &mut Framebuffer, view_matrix: &Matrix) {
        for (i, star) in self.stars.iter().enumerate() {
            let screen_pos = self.world_to_screen(*star, view_matrix);
            
            if screen_pos.x >= 0.0 && screen_pos.x < self.screen_width &&
               screen_pos.y >= 0.0 && screen_pos.y < self.screen_height {
                let x = screen_pos.x as u32;
                let y = screen_pos.y as u32;
                
                let brightness = ((i * 73) % 100) as f32 / 100.0;
                let star_color = Color::new(
                    (200.0 + brightness * 55.0) as u8,
                    (200.0 + brightness * 55.0) as u8,
                    (220.0 + brightness * 35.0) as u8,
                    255
                );
                
                framebuffer.set_pixel_color(x, y, star_color);
                
                // Glow for bright stars
                if brightness > 0.7 {
                    if x > 0 { framebuffer.set_pixel_color(x - 1, y, star_color); }
                    if x < framebuffer.width - 1 { framebuffer.set_pixel_color(x + 1, y, star_color); }
                    if y > 0 { framebuffer.set_pixel_color(x, y - 1, star_color); }
                    if y < framebuffer.height - 1 { framebuffer.set_pixel_color(x, y + 1, star_color); }
                }
            }
        }
    }

    // ===== CELESTIAL BODIES =====

    fn render_celestial_body_to_framebuffer(
        &self,
        framebuffer: &mut Framebuffer,
        body: &CelestialBody,
        view_matrix: &Matrix,
    ) {
        if let Some(ref model) = self.sphere_model {
            self.render_obj_model_to_framebuffer(framebuffer, model, body, view_matrix);
        }
    }

    fn render_obj_model_to_framebuffer(
        &self,
        framebuffer: &mut Framebuffer,
        model: &ObjModel,
        body: &CelestialBody,
        view_matrix: &Matrix,
    ) {
        // Transformation matrices
        let scale_matrix = create_scale_matrix(body.radius, body.radius, body.radius);
        let translation_matrix = create_translation_matrix(body.position.x, body.position.y, body.position.z);
        let model_matrix = translation_matrix.multiply(&scale_matrix);
        
        // LOD calculation
        let distance_to_camera = Self::vector_length(body.position);
        let apparent_radius = (body.radius * self.screen_width) / (distance_to_camera * 2.0);
        
        if apparent_radius < 0.3 {
            return; // Too small, skip
        }
        
        // Adaptive LOD
        let face_skip = if apparent_radius < 3.0 { 8 } 
                       else if apparent_radius < 8.0 { 4 } 
                       else if apparent_radius < 20.0 { 2 } 
                       else { 1 };

        // Process each face
        for (face_idx, face) in model.faces.iter().enumerate() {
            if face_skip > 1 && face_idx % face_skip != 0 {
                continue; // LOD skip
            }
            
            if face.vertices.len() < 3 {
                continue;
            }

            // Triangulate face
            for i in 1..face.vertices.len() - 1 {
                let v0_idx = face.vertices[0];
                let v1_idx = face.vertices[i];
                let v2_idx = face.vertices[i + 1];

                if v0_idx >= model.vertices.len() || v1_idx >= model.vertices.len() || v2_idx >= model.vertices.len() {
                    continue;
                }

                let v0 = model.vertices[v0_idx];
                let v1 = model.vertices[v1_idx];
                let v2 = model.vertices[v2_idx];

                // Vertex shader: transform vertices
                let v0_out = vertex_shader(
                    v0, Vector3::new(v0.x, v0.y, v0.z).normalized(), body.color,
                    &model_matrix, view_matrix, &self.projection_matrix, &self.viewport_matrix
                );
                let v1_out = vertex_shader(
                    v1, Vector3::new(v1.x, v1.y, v1.z).normalized(), body.color,
                    &model_matrix, view_matrix, &self.projection_matrix, &self.viewport_matrix
                );
                let v2_out = vertex_shader(
                    v2, Vector3::new(v2.x, v2.y, v2.z).normalized(), body.color,
                    &model_matrix, view_matrix, &self.projection_matrix, &self.viewport_matrix
                );

                // Backface culling
                let edge1_x = v1_out.screen_position.x - v0_out.screen_position.x;
                let edge1_y = v1_out.screen_position.y - v0_out.screen_position.y;
                let edge2_x = v2_out.screen_position.x - v0_out.screen_position.x;
                let edge2_y = v2_out.screen_position.y - v0_out.screen_position.y;
                let cross_z = edge1_x * edge2_y - edge1_y * edge2_x;
                
                if cross_z <= 0.0 {
                    continue; // Back-facing
                }

                // Rasterize triangle usando el módulo rasterizer optimizado
                // Este módulo ahora incluye frustum culling de fragmentos
                self.rasterize_triangle_to_framebuffer(
                    framebuffer,
                    v0_out,
                    v1_out,
                    v2_out,
                );
            }
        }
    }

    /// Rasteriza un triángulo al framebuffer
    /// 
    /// Esta función usa el módulo rasterizer optimizado que incluye:
    /// - Frustum culling de fragmentos (no genera fragmentos fuera de pantalla)
    /// - Interpolación baricéntrica de atributos
    /// - Fragment shader para calcular colores finales
    fn rasterize_triangle_to_framebuffer(
        &self,
        framebuffer: &mut Framebuffer,
        v0: VertexShaderOutput,
        v1: VertexShaderOutput,
        v2: VertexShaderOutput,
    ) {
        // Crear el triángulo desde los vértices transformados
        let triangle = Triangle::new(v0, v1, v2);
        
        // Rasterizar usando el módulo optimizado con frustum culling
        // Ahora pasamos screen_width y screen_height para que el rasterizer
        // pueda clipear fragmentos fuera de pantalla ANTES de generarlos
        let fragments = rasterize_triangle(
            &triangle, 
            self.screen_width, 
            self.screen_height
        );
        
        // Procesar cada fragmento generado (todos están garantizados dentro de pantalla)
        for fragment in fragments {
            // Fragment shader: calcular color final con iluminación
            let final_color = fragment_shader(
                fragment.color, 
                fragment.normal, 
                &self.lighting_config
            );
            
            // Escribir al framebuffer (sin necesidad de verificar límites)
            framebuffer.set_pixel_color(
                fragment.screen_x as u32, 
                fragment.screen_y as u32, 
                final_color
            );
        }
    }

    // ===== ORBITS =====

    fn render_orbits_to_framebuffer(
        &self,
        framebuffer: &mut Framebuffer,
        solar_system: &SolarSystem,
        view_matrix: &Matrix,
    ) {
        for planet in &solar_system.planets {
            let radius = planet.orbital_radius;
            let segments = 120;
            
            let orbit_color = Color::new(
                (planet.color.r as f32 * 0.5) as u8,
                (planet.color.g as f32 * 0.5) as u8,
                (planet.color.b as f32 * 0.5) as u8,
                160
            );

            for i in 0..segments {
                let angle1 = (i as f32 / segments as f32) * 2.0 * PI;
                let angle2 = ((i + 1) as f32 / segments as f32) * 2.0 * PI;

                let p1 = Vector3::new(radius * angle1.cos(), 0.0, radius * angle1.sin());
                let p2 = Vector3::new(radius * angle2.cos(), 0.0, radius * angle2.sin());

                let screen1 = self.world_to_screen(p1, view_matrix);
                let screen2 = self.world_to_screen(p2, view_matrix);

                if (screen1.x >= -30.0 && screen1.x < self.screen_width + 30.0 &&
                    screen1.y >= -30.0 && screen1.y < self.screen_height + 30.0) ||
                   (screen2.x >= -30.0 && screen2.x < self.screen_width + 30.0 &&
                    screen2.y >= -30.0 && screen2.y < self.screen_height + 30.0) {
                    
                    framebuffer.set_current_color(orbit_color);
                    framebuffer.draw_line(
                        screen1.x as i32, screen1.y as i32,
                        screen2.x as i32, screen2.y as i32
                    );
                }
            }
        }
    }

    // ===== UTILITY =====

    fn world_to_screen(&self, world_pos: Vector3, view_matrix: &Matrix) -> Vector3 {
        let view_pos = view_matrix.transform_point(world_pos);
        let proj_pos = self.projection_matrix.transform_point(view_pos);
        self.viewport_matrix.transform_point(proj_pos)
    }

    fn vector_length(v: Vector3) -> f32 {
        (v.x * v.x + v.y * v.y + v.z * v.z).sqrt()
    }

    pub fn toggle_wireframe(&mut self) {
        self.show_wireframe = !self.show_wireframe;
    }

    pub fn resize(&mut self, new_width: f32, new_height: f32) {
        self.screen_width = new_width;
        self.screen_height = new_height;
        self.projection_matrix = create_projection_matrix(PI / 4.0, new_width / new_height, 0.1, 1000.0);
        self.viewport_matrix = create_viewport_matrix(0.0, 0.0, new_width, new_height);
    }
}
