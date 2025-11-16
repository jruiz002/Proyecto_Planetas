#![allow(dead_code)]
use crate::camera::Camera;
use crate::celestial_body::{CelestialBody, CelestialBodyType};
use crate::matrix::{create_projection_matrix, create_viewport_matrix, create_translation_matrix, create_scale_matrix, Matrix, multiply_matrix_vector4};
use crate::solar_system::SolarSystem;
use crate::framebuffer::Framebuffer;
use crate::obj_loader::ObjModel;
use rand::Rng;
use raylib::prelude::*;
use std::f32::consts::PI;

#[derive(Clone, Copy)]
struct Vertex3D {
    position: Vector3,
    normal: Vector3,
    color: Color,
}

#[derive(Clone, Copy)]
struct Triangle3D {
    vertices: [Vertex3D; 3],
}

struct SphereGeometry {
    triangles: Vec<Triangle3D>,
    center: Vector3,
    radius: f32,
}

// Helper function to calculate vector length
fn vector_length(v: Vector3) -> f32 {
    (v.x * v.x + v.y * v.y + v.z * v.z).sqrt()
}

// Helper function to normalize a vector
fn normalize_vector(v: Vector3) -> Vector3 {
    let length = vector_length(v);
    if length > 0.0 {
        Vector3::new(v.x / length, v.y / length, v.z / length)
    } else {
        Vector3::new(0.0, 0.0, 0.0)
    }
}

pub struct Renderer {
    pub screen_width: f32,
    pub screen_height: f32,
    pub projection_matrix: Matrix,
    pub viewport_matrix: Matrix,
    pub stars: Vec<Vector3>, // For skybox
    pub show_wireframe: bool,
    pub light_direction: Vector3, // Directional light (sun)
    pub ambient_light: f32,
    pub sphere_detail: usize, // Level of detail for spheres
    pub sphere_model: Option<ObjModel>, // Sphere model from OBJ file
}

impl Renderer {
    pub fn new(screen_width: f32, screen_height: f32) -> Self {
        let projection_matrix = create_projection_matrix(
            PI / 4.0,                     // 45 degree FOV
            screen_width / screen_height, // aspect ratio
            0.1,                          // near plane
            1000.0,                       // far plane
        );

        let viewport_matrix = create_viewport_matrix(0.0, 0.0, screen_width, screen_height);

        // Load sphere model from OBJ file
        let sphere_model = match ObjModel::load("assets/sphere.obj") {
            Ok(model) => {
                println!("Sphere model loaded successfully with {} vertices", model.vertices.len());
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
            light_direction: normalize_vector(Vector3::new(-1.0, -1.0, -1.0)), // Light from sun
            ambient_light: 0.2,                                                // 20% ambient light
            sphere_detail: 20, // Medium detail level
            sphere_model,
        };

        renderer.generate_stars(1500); // More stars for better skybox
        renderer
    }

    fn generate_stars(&mut self, count: usize) {
        let mut rng = rand::thread_rng();
        self.stars.clear();

        for _ in 0..count {
            // Generate stars on a sphere around the solar system
            let theta = rng.gen::<f32>() * 2.0 * PI;
            let phi = rng.gen::<f32>() * PI;
            let distance = 800.0 + rng.gen::<f32>() * 200.0; // Far away

            let x = distance * phi.sin() * theta.cos();
            let y = distance * phi.cos();
            let z = distance * phi.sin() * theta.sin();

            self.stars.push(Vector3::new(x, y, z));
        }
    }

    // Generate a 3D sphere with proper geometry
    fn generate_sphere(&self, center: Vector3, radius: f32, base_color: Color) -> SphereGeometry {
        let mut triangles = Vec::new();
        let detail = self.sphere_detail;

        // Generate sphere using UV sphere method
        for i in 0..detail {
            for j in 0..detail {
                let u1 = i as f32 / detail as f32;
                let u2 = (i + 1) as f32 / detail as f32;
                let v1 = j as f32 / detail as f32;
                let v2 = (j + 1) as f32 / detail as f32;

                // Convert UV to spherical coordinates
                let p1 = self.uv_to_sphere(u1, v1, center, radius);
                let p2 = self.uv_to_sphere(u2, v1, center, radius);
                let p3 = self.uv_to_sphere(u1, v2, center, radius);
                let p4 = self.uv_to_sphere(u2, v2, center, radius);

                // Calculate normals (pointing outward from center)
                let n1 = normalize_vector(Vector3::new(
                    p1.x - center.x,
                    p1.y - center.y,
                    p1.z - center.z,
                ));
                let n2 = normalize_vector(Vector3::new(
                    p2.x - center.x,
                    p2.y - center.y,
                    p2.z - center.z,
                ));
                let n3 = normalize_vector(Vector3::new(
                    p3.x - center.x,
                    p3.y - center.y,
                    p3.z - center.z,
                ));
                let n4 = normalize_vector(Vector3::new(
                    p4.x - center.x,
                    p4.y - center.y,
                    p4.z - center.z,
                ));

                // Calculate lighting for each vertex
                let c1 = self.calculate_vertex_color(base_color, n1);
                let c2 = self.calculate_vertex_color(base_color, n2);
                let c3 = self.calculate_vertex_color(base_color, n3);
                let c4 = self.calculate_vertex_color(base_color, n4);

                // Create vertices
                let v1 = Vertex3D {
                    position: p1,
                    normal: n1,
                    color: c1,
                };
                let v2 = Vertex3D {
                    position: p2,
                    normal: n2,
                    color: c2,
                };
                let v3 = Vertex3D {
                    position: p3,
                    normal: n3,
                    color: c3,
                };
                let v4 = Vertex3D {
                    position: p4,
                    normal: n4,
                    color: c4,
                };

                // Create two triangles for each quad
                triangles.push(Triangle3D {
                    vertices: [v1, v2, v3],
                });
                triangles.push(Triangle3D {
                    vertices: [v2, v4, v3],
                });
            }
        }

        SphereGeometry {
            triangles,
            center,
            radius,
        }
    }

    fn uv_to_sphere(&self, u: f32, v: f32, center: Vector3, radius: f32) -> Vector3 {
        let theta = u * 2.0 * PI; // Longitude
        let phi = v * PI; // Latitude

        let x = center.x + radius * phi.sin() * theta.cos();
        let y = center.y + radius * phi.cos();
        let z = center.z + radius * phi.sin() * theta.sin();

        Vector3::new(x, y, z)
    }

    fn calculate_vertex_color(&self, base_color: Color, normal: Vector3) -> Color {
        // Calculate diffuse lighting
        let dot_product = -(normal.x * self.light_direction.x
            + normal.y * self.light_direction.y
            + normal.z * self.light_direction.z);
        let diffuse = dot_product.max(0.0);

        // Combine ambient and diffuse lighting
        let lighting = self.ambient_light + (1.0 - self.ambient_light) * diffuse;

        Color::new(
            (base_color.r as f32 * lighting) as u8,
            (base_color.g as f32 * lighting) as u8,
            (base_color.b as f32 * lighting) as u8,
            base_color.a,
        )
    }

    pub fn render_skybox(&self, d: &mut RaylibDrawHandle, camera: &Camera) {
        let view_matrix = camera.get_view_matrix();

        // Render stars with varying sizes and brightness
        for (i, star) in self.stars.iter().enumerate() {
            let screen_pos = self.world_to_screen(*star, &view_matrix);
            if screen_pos.x >= 0.0
                && screen_pos.x < self.screen_width
                && screen_pos.y >= 0.0
                && screen_pos.y < self.screen_height
            {
                // Vary star brightness and size
                let brightness = 100 + (i % 156) as u8; // Pseudo-random brightness
                let size = if i % 50 == 0 { 2 } else { 1 }; // Some stars are bigger

                let star_color = Color::new(brightness, brightness, brightness, 255);

                if size > 1 {
                    d.draw_circle(
                        screen_pos.x as i32,
                        screen_pos.y as i32,
                        size as f32,
                        star_color,
                    );
                } else {
                    d.draw_pixel(screen_pos.x as i32, screen_pos.y as i32, star_color);
                }

                // Add twinkle effect for bright stars
                if brightness > 200 && i % 100 == 0 {
                    let twinkle_color = Color::new(255, 255, 200, 100);
                    d.draw_circle(screen_pos.x as i32, screen_pos.y as i32, 3.0, twinkle_color);
                }
            }
        }

        // Add nebula-like background gradient
        self.render_space_gradient(d);
    }

    fn render_space_gradient(&self, d: &mut RaylibDrawHandle) {
        // Create a subtle space gradient background
        let center_x = self.screen_width / 2.0;
        let center_y = self.screen_height / 2.0;
        let max_distance = (center_x * center_x + center_y * center_y).sqrt();

        // Draw gradient circles from center outward
        for i in 0..20 {
            let radius = (i as f32 / 19.0) * max_distance;
            let alpha = (20 - i) as u8 * 2; // Fade out towards edges
            let color = Color::new(5, 5, 15, alpha); // Deep space blue

            d.draw_circle_lines(center_x as i32, center_y as i32, radius, color);
        }
    }

    pub fn render_solar_system(
        &self,
        d: &mut RaylibDrawHandle,
        solar_system: &SolarSystem,
        camera: &Camera,
        show_orbits: bool,
    ) {
        let view_matrix = camera.get_view_matrix();

        // Render orbits first (behind everything)
        if show_orbits {
            self.render_orbits(d, solar_system, &view_matrix);
        }

        // Get bodies sorted by distance for proper depth rendering
        let bodies = solar_system.get_bodies_sorted_by_distance(camera.get_position());

        // Render all celestial bodies
        for body in bodies {
            self.render_celestial_body(d, body, &view_matrix);

            // Render moons
            for moon in &body.moons {
                self.render_celestial_body(d, moon, &view_matrix);
            }
        }
    }

    // Nuevo método para renderizar al framebuffer
    pub fn render_solar_system_to_framebuffer(
        &self,
        framebuffer: &mut Framebuffer,
        solar_system: &SolarSystem,
        camera: &Camera,
        show_orbits: bool,
    ) {
        let view_matrix = camera.get_view_matrix();

        // Render background gradient to framebuffer
        self.render_space_gradient_to_framebuffer(framebuffer);

        // Render stars to framebuffer
        self.render_stars_to_framebuffer(framebuffer, &view_matrix);

        // Get bodies sorted by distance for proper depth rendering
        let bodies = solar_system.get_bodies_sorted_by_distance(camera.get_position());

        // Render all celestial bodies to framebuffer
        for body in bodies {
            self.render_celestial_body_to_framebuffer(framebuffer, body, &view_matrix);

            // Render moons
            for moon in &body.moons {
                self.render_celestial_body_to_framebuffer(framebuffer, moon, &view_matrix);
            }
        }

        // Render orbits if enabled (as lines in the framebuffer)
        if show_orbits {
            self.render_orbits_to_framebuffer(framebuffer, solar_system, &view_matrix);
        }
    }

    fn render_celestial_body(
        &self,
        d: &mut RaylibDrawHandle,
        body: &CelestialBody,
        view_matrix: &Matrix,
    ) {
        // Calculate distance for level of detail
        let distance_to_camera = vector_length(body.position);
        let apparent_radius = (body.radius * self.screen_width) / (distance_to_camera * 2.0);

        if apparent_radius < 2.0 {
            // Too small, render as enhanced point
            let center_screen = self.world_to_screen(body.position, view_matrix);
            if self.is_point_on_screen(center_screen) {
                // Enhanced point rendering with glow for distant objects
                let glow_color = Color::new(body.color.r, body.color.g, body.color.b, 100);
                d.draw_circle(
                    center_screen.x as i32,
                    center_screen.y as i32,
                    2.0,
                    glow_color,
                );
                d.draw_pixel(center_screen.x as i32, center_screen.y as i32, body.color);
            }
            return;
        }

        // Generate enhanced sphere color based on body type
        let enhanced_color = self.enhance_planet_color(body);

        // Generate 3D sphere geometry
        let sphere = self.generate_sphere(body.position, body.radius, enhanced_color);

        // Render the sphere using triangles
        self.render_sphere_geometry(d, &sphere, view_matrix);

        // Add special effects based on body type
        match body.body_type {
            CelestialBodyType::Star => {
                self.render_star_effects(d, body, view_matrix, apparent_radius);
            }
            CelestialBodyType::Planet => {
                self.render_planet_effects(d, body, view_matrix, apparent_radius);
            }
            CelestialBodyType::Moon => {
                self.render_moon_effects(d, body, view_matrix, apparent_radius);
            }
        }

        // Render rings if the body has them
        if body.has_rings {
            self.render_enhanced_rings(d, body, view_matrix, apparent_radius);
        }

        // Render enhanced name label
        if apparent_radius > 15.0 {
            self.render_enhanced_label(d, body, view_matrix, apparent_radius);
        }
    }

    fn enhance_planet_color(&self, body: &CelestialBody) -> Color {
        // Enhance colors based on planet characteristics
        match body.name.as_str() {
            "Sol" => Color::new(255, 220, 100, 255), // Bright yellow-orange
            "Pyrion" => Color::new(220, 50, 30, 255), // Volcanic red-orange
            "Verdania" => Color::new(50, 150, 100, 255), // Earth-like green-blue
            "Gigantus" => Color::new(200, 120, 50, 255), // Gas giant orange-brown
            "Glacialis" => Color::new(150, 200, 255, 255), // Ice blue
            "Plutonix" => Color::new(120, 120, 140, 255), // Rocky gray-blue
            _ => body.color,                         // Default color for moons
        }
    }

    fn render_sphere_geometry(
        &self,
        d: &mut RaylibDrawHandle,
        sphere: &SphereGeometry,
        view_matrix: &Matrix,
    ) {
        // Sort triangles by depth for proper rendering
        let mut triangles_with_depth: Vec<(f32, &Triangle3D)> = sphere
            .triangles
            .iter()
            .map(|tri| {
                // Calculate average depth of triangle
                let avg_z = (tri.vertices[0].position.z
                    + tri.vertices[1].position.z
                    + tri.vertices[2].position.z)
                    / 3.0;
                (avg_z, tri)
            })
            .collect();

        // Sort by depth (far to near)
        triangles_with_depth.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

        // Render triangles
        for (_, triangle) in triangles_with_depth {
            if self.show_wireframe {
                self.render_triangle_wireframe(d, triangle, view_matrix);
            } else {
                self.render_triangle_filled(d, triangle, view_matrix);
            }
        }
    }

    fn render_triangle_filled(
        &self,
        d: &mut RaylibDrawHandle,
        triangle: &Triangle3D,
        view_matrix: &Matrix,
    ) {
        // Transform vertices to screen space
        let screen_vertices: Vec<Vector3> = triangle
            .vertices
            .iter()
            .map(|v| self.world_to_screen(v.position, view_matrix))
            .collect();

        // Check if triangle is on screen
        if screen_vertices.iter().any(|v| self.is_point_on_screen(*v)) {
            // Use average color of the three vertices
            let avg_color = self.average_colors(&[
                triangle.vertices[0].color,
                triangle.vertices[1].color,
                triangle.vertices[2].color,
            ]);

            // Draw filled triangle
            d.draw_triangle(
                Vector2::new(screen_vertices[0].x, screen_vertices[0].y),
                Vector2::new(screen_vertices[1].x, screen_vertices[1].y),
                Vector2::new(screen_vertices[2].x, screen_vertices[2].y),
                avg_color,
            );
        }
    }

    fn render_triangle_wireframe(
        &self,
        d: &mut RaylibDrawHandle,
        triangle: &Triangle3D,
        view_matrix: &Matrix,
    ) {
        // Transform vertices to screen space
        let screen_vertices: Vec<Vector3> = triangle
            .vertices
            .iter()
            .map(|v| self.world_to_screen(v.position, view_matrix))
            .collect();

        // Draw triangle edges
        let color = Color::WHITE;
        d.draw_line(
            screen_vertices[0].x as i32,
            screen_vertices[0].y as i32,
            screen_vertices[1].x as i32,
            screen_vertices[1].y as i32,
            color,
        );
        d.draw_line(
            screen_vertices[1].x as i32,
            screen_vertices[1].y as i32,
            screen_vertices[2].x as i32,
            screen_vertices[2].y as i32,
            color,
        );
        d.draw_line(
            screen_vertices[2].x as i32,
            screen_vertices[2].y as i32,
            screen_vertices[0].x as i32,
            screen_vertices[0].y as i32,
            color,
        );
    }

    fn average_colors(&self, colors: &[Color]) -> Color {
        let r = colors.iter().map(|c| c.r as u32).sum::<u32>() / colors.len() as u32;
        let g = colors.iter().map(|c| c.g as u32).sum::<u32>() / colors.len() as u32;
        let b = colors.iter().map(|c| c.b as u32).sum::<u32>() / colors.len() as u32;
        let a = colors.iter().map(|c| c.a as u32).sum::<u32>() / colors.len() as u32;

        Color::new(r as u8, g as u8, b as u8, a as u8)
    }

    fn render_rings(
        &self,
        d: &mut RaylibDrawHandle,
        body: &CelestialBody,
        view_matrix: &Matrix,
        center_screen: Vector3,
        body_radius: f32,
    ) {
        let distance_to_camera = vector_length(body.position);
        let inner_radius =
            (body.ring_inner_radius * self.screen_width) / (distance_to_camera * 2.0);
        let outer_radius =
            (body.ring_outer_radius * self.screen_width) / (distance_to_camera * 2.0);

        // Draw rings as ellipses (simplified)
        for i in 0..3 {
            let radius = inner_radius + (outer_radius - inner_radius) * (i as f32 / 2.0);
            let alpha = (body.ring_color.a as f32 * (1.0 - i as f32 * 0.3)) as u8;
            let ring_color = Color::new(
                body.ring_color.r,
                body.ring_color.g,
                body.ring_color.b,
                alpha,
            );

            d.draw_circle_lines(
                center_screen.x as i32,
                center_screen.y as i32,
                radius,
                ring_color,
            );
        }
    }

    fn render_orbits(
        &self,
        d: &mut RaylibDrawHandle,
        solar_system: &SolarSystem,
        view_matrix: &Matrix,
    ) {
        for planet in &solar_system.planets {
            if planet.orbital_radius > 0.0 {
                let orbit_points = planet.get_orbit_points(64);
                let mut screen_points = Vec::new();

                for point in orbit_points {
                    let screen_point = self.world_to_screen(point, view_matrix);
                    screen_points.push(screen_point);
                }

                // Draw orbit as connected lines
                for i in 0..screen_points.len() {
                    let current = screen_points[i];
                    let next = screen_points[(i + 1) % screen_points.len()];

                    if self.is_point_on_screen(current) || self.is_point_on_screen(next) {
                        d.draw_line(
                            current.x as i32,
                            current.y as i32,
                            next.x as i32,
                            next.y as i32,
                            Color::new(100, 100, 100, 100), // Semi-transparent gray
                        );
                    }
                }
            }
        }
    }

    fn world_to_screen(&self, world_pos: Vector3, view_matrix: &Matrix) -> Vector3 {
        // Transform world position through view, projection, and viewport matrices
        let view_pos = view_matrix.transform_point(world_pos);
        let proj_pos = self.projection_matrix.transform_point(view_pos);
        let screen_pos = self.viewport_matrix.transform_point(proj_pos);
        screen_pos
    }

    fn render_star_effects(
        &self,
        d: &mut RaylibDrawHandle,
        body: &CelestialBody,
        view_matrix: &Matrix,
        apparent_radius: f32,
    ) {
        let center_screen = self.world_to_screen(body.position, view_matrix);

        // Bright glow effect for stars
        let glow_color = Color::new(body.color.r, body.color.g, body.color.b, 80);
        d.draw_circle(
            center_screen.x as i32,
            center_screen.y as i32,
            apparent_radius * 2.0,
            glow_color,
        );

        // Corona effect
        let corona_color = Color::new(255, 255, 200, 40);
        d.draw_circle(
            center_screen.x as i32,
            center_screen.y as i32,
            apparent_radius * 3.0,
            corona_color,
        );

        // Solar flares (simple lines radiating outward)
        for i in 0..8 {
            let angle = (i as f32 * 45.0) * std::f32::consts::PI / 180.0;
            let start_x = center_screen.x + angle.cos() * apparent_radius * 1.2;
            let start_y = center_screen.y + angle.sin() * apparent_radius * 1.2;
            let end_x = center_screen.x + angle.cos() * apparent_radius * 2.5;
            let end_y = center_screen.y + angle.sin() * apparent_radius * 2.5;

            d.draw_line(
                start_x as i32,
                start_y as i32,
                end_x as i32,
                end_y as i32,
                Color::new(255, 255, 150, 120),
            );
        }
    }

    fn render_planet_effects(
        &self,
        d: &mut RaylibDrawHandle,
        body: &CelestialBody,
        view_matrix: &Matrix,
        apparent_radius: f32,
    ) {
        let center_screen = self.world_to_screen(body.position, view_matrix);

        // Atmospheric glow for planets
        let atmosphere_color = match body.name.as_str() {
            "Verdania" => Color::new(100, 150, 255, 60), // Blue atmosphere
            "Gigantus" => Color::new(200, 150, 100, 40), // Thick atmosphere
            "Glacialis" => Color::new(200, 220, 255, 50), // Icy atmosphere
            _ => Color::new(body.color.r, body.color.g, body.color.b, 30),
        };

        d.draw_circle(
            center_screen.x as i32,
            center_screen.y as i32,
            apparent_radius * 1.3,
            atmosphere_color,
        );

        // Add surface details for larger planets
        if apparent_radius > 20.0 {
            self.render_surface_details(d, body, center_screen, apparent_radius);
        }
    }

    fn render_moon_effects(
        &self,
        d: &mut RaylibDrawHandle,
        body: &CelestialBody,
        view_matrix: &Matrix,
        apparent_radius: f32,
    ) {
        let center_screen = self.world_to_screen(body.position, view_matrix);

        // Subtle glow for moons
        let glow_color = Color::new(body.color.r, body.color.g, body.color.b, 40);
        d.draw_circle(
            center_screen.x as i32,
            center_screen.y as i32,
            apparent_radius * 1.1,
            glow_color,
        );

        // Add crater effects for larger moons
        if apparent_radius > 15.0 {
            self.render_crater_effects(d, center_screen, apparent_radius);
        }
    }

    fn render_enhanced_rings(
        &self,
        d: &mut RaylibDrawHandle,
        body: &CelestialBody,
        view_matrix: &Matrix,
        apparent_radius: f32,
    ) {
        let center_screen = self.world_to_screen(body.position, view_matrix);

        // Multiple ring layers with different colors and transparencies
        let ring_colors = [
            Color::new(200, 180, 120, 100),
            Color::new(180, 160, 100, 80),
            Color::new(160, 140, 80, 60),
        ];

        for (i, &ring_color) in ring_colors.iter().enumerate() {
            let ring_radius = apparent_radius * (2.0 + i as f32 * 0.5);
            let ring_thickness = 3.0 + i as f32;

            d.draw_circle_lines(
                center_screen.x as i32,
                center_screen.y as i32,
                ring_radius,
                ring_color,
            );

            // Add inner ring detail
            d.draw_circle_lines(
                center_screen.x as i32,
                center_screen.y as i32,
                ring_radius - ring_thickness,
                ring_color,
            );
        }

        // Add ring particles for close-up view
        if apparent_radius > 30.0 {
            self.render_ring_particles(d, center_screen, apparent_radius);
        }
    }

    fn render_enhanced_label(
        &self,
        d: &mut RaylibDrawHandle,
        body: &CelestialBody,
        view_matrix: &Matrix,
        apparent_radius: f32,
    ) {
        let center_screen = self.world_to_screen(body.position, view_matrix);

        // Enhanced label with background and better positioning
        let label_y = (center_screen.y + apparent_radius + 20.0) as i32;
        let text_width = body.name.len() as f32 * 8.0;
        let text_x = (center_screen.x - text_width / 2.0) as i32;

        // Draw background rectangle
        d.draw_rectangle(
            text_x - 5,
            label_y - 2,
            text_width as i32 + 10,
            16,
            Color::new(0, 0, 0, 120),
        );

        // Draw text with enhanced visibility
        d.draw_text(&body.name, text_x, label_y, 14, Color::WHITE);

        // Add distance information for planets
        if matches!(body.body_type, CelestialBodyType::Planet) {
            let distance = vector_length(body.position);
            let distance_text = format!("{:.1} AU", distance / 50.0); // Approximate AU conversion
            let distance_y = label_y + 18;
            let distance_width = distance_text.len() as f32 * 6.0;
            let distance_x = (center_screen.x - distance_width / 2.0) as i32;

            d.draw_text(
                &distance_text,
                distance_x,
                distance_y,
                10,
                Color::new(200, 200, 200, 255),
            );
        }
    }

    fn render_surface_details(
        &self,
        d: &mut RaylibDrawHandle,
        body: &CelestialBody,
        center: Vector3,
        radius: f32,
    ) {
        // Add surface features based on planet type
        match body.name.as_str() {
            "Verdania" => {
                // Continents and oceans
                for i in 0..3 {
                    let angle = i as f32 * 120.0 * std::f32::consts::PI / 180.0;
                    let x = center.x + angle.cos() * radius * 0.6;
                    let y = center.y + angle.sin() * radius * 0.6;
                    d.draw_circle(
                        x as i32,
                        y as i32,
                        radius * 0.3,
                        Color::new(100, 200, 100, 180),
                    );
                }
            }
            "Pyrion" => {
                // Volcanic activity
                for i in 0..5 {
                    let angle = i as f32 * 72.0 * std::f32::consts::PI / 180.0;
                    let x = center.x + angle.cos() * radius * 0.7;
                    let y = center.y + angle.sin() * radius * 0.7;
                    d.draw_circle(
                        x as i32,
                        y as i32,
                        radius * 0.15,
                        Color::new(255, 100, 50, 200),
                    );
                }
            }
            "Gigantus" => {
                // Gas bands
                for i in 0..4 {
                    let band_y = center.y - radius + (i as f32 * radius * 0.5);
                    d.draw_line(
                        (center.x - radius) as i32,
                        band_y as i32,
                        (center.x + radius) as i32,
                        band_y as i32,
                        Color::new(150, 100, 50, 150),
                    );
                }
            }
            _ => {}
        }
    }

    fn render_crater_effects(&self, d: &mut RaylibDrawHandle, center: Vector3, radius: f32) {
        // Simple crater representation
        for i in 0..4 {
            let angle = i as f32 * 90.0 * std::f32::consts::PI / 180.0;
            let x = center.x + angle.cos() * radius * 0.6;
            let y = center.y + angle.sin() * radius * 0.6;
            d.draw_circle(
                x as i32,
                y as i32,
                radius * 0.2,
                Color::new(80, 80, 80, 150),
            );
        }
    }

    fn render_ring_particles(&self, d: &mut RaylibDrawHandle, center: Vector3, radius: f32) {
        // Render individual ring particles for detailed view
        for i in 0..20 {
            let angle = i as f32 * 18.0 * std::f32::consts::PI / 180.0;
            let ring_radius = radius * (2.0 + (i % 3) as f32 * 0.3);
            let x = center.x + angle.cos() * ring_radius;
            let y = center.y + angle.sin() * ring_radius;

            d.draw_pixel(x as i32, y as i32, Color::new(200, 180, 120, 180));
        }
    }

    fn is_point_on_screen(&self, point: Vector3) -> bool {
        point.x >= 0.0
            && point.x < self.screen_width
            && point.y >= 0.0
            && point.y < self.screen_height
    }

    pub fn toggle_wireframe(&mut self) {
        self.show_wireframe = !self.show_wireframe;
    }

    pub fn resize(&mut self, new_width: f32, new_height: f32) {
        self.screen_width = new_width;
        self.screen_height = new_height;

        self.projection_matrix =
            create_projection_matrix(PI / 4.0, new_width / new_height, 0.1, 1000.0);

        self.viewport_matrix = create_viewport_matrix(0.0, 0.0, new_width, new_height);
    }

    // ===== Métodos para renderizar al framebuffer =====

    fn render_space_gradient_to_framebuffer(&self, framebuffer: &mut Framebuffer) {
        // Optimized gradient with pixel skipping for massive performance boost
        let width = framebuffer.width as f32;
        let height = framebuffer.height as f32;
        let center_x = width * 0.5;
        let center_y = height * 0.5;
        let max_dist = ((center_x * center_x) + (center_y * center_y)).sqrt();
        
        // Render at 2x2 pixel blocks for 4x speedup
        let pixel_step = 2;
        
        for y in (0..framebuffer.height).step_by(pixel_step) {
            let dy = y as f32 - center_y;
            for x in (0..framebuffer.width).step_by(pixel_step) {
                let dx = x as f32 - center_x;
                let distance = (dx * dx + dy * dy).sqrt();
                let norm_dist = (distance / max_dist).min(1.0);
                
                // Simplified gradient (removed spiral for performance)
                let base_intensity = norm_dist * 0.8;
                
                // Deep space colors
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
                
                // Vary star brightness and size
                let brightness = ((i * 73) % 100) as f32 / 100.0; // Pseudo-random brightness
                let star_color = Color::new(
                    (200.0 + brightness * 55.0) as u8,
                    (200.0 + brightness * 55.0) as u8,
                    (220.0 + brightness * 35.0) as u8,
                    255
                );
                
                // Draw star with small cross pattern for larger stars
                framebuffer.set_pixel_color(x, y, star_color);
                
                if brightness > 0.7 {
                    // Add glow effect for bright stars
                    if x > 0 { framebuffer.set_pixel_color(x - 1, y, star_color); }
                    if x < framebuffer.width - 1 { framebuffer.set_pixel_color(x + 1, y, star_color); }
                    if y > 0 { framebuffer.set_pixel_color(x, y - 1, star_color); }
                    if y < framebuffer.height - 1 { framebuffer.set_pixel_color(x, y + 1, star_color); }
                }
            }
        }
    }

    fn render_celestial_body_to_framebuffer(
        &self,
        framebuffer: &mut Framebuffer,
        body: &CelestialBody,
        view_matrix: &Matrix,
    ) {
        // Render the sphere without glow effect
        if let Some(ref model) = self.sphere_model {
            self.render_obj_model_to_framebuffer(framebuffer, model, body, view_matrix);
        } else {
            // Fallback: render as a simple circle
            self.render_simple_sphere_to_framebuffer(framebuffer, body, view_matrix);
        }
    }

    fn render_sun_glow_to_framebuffer(
        &self,
        framebuffer: &mut Framebuffer,
        body: &CelestialBody,
        view_matrix: &Matrix,
    ) {
        let screen_pos = self.world_to_screen(body.position, view_matrix);
        let distance = vector_length(body.position);
        let apparent_radius = (body.radius * self.screen_width) / (distance * 2.0);
        
        let center_x = screen_pos.x as i32;
        let center_y = screen_pos.y as i32;
        
        // Enhanced multi-layer glow with better color blending
        for layer in 0..6 {
            let glow_radius = (apparent_radius * (1.8 + layer as f32 * 0.4)) as i32;
            let layer_intensity = (6 - layer) as f32 / 6.0;
            
            // Limit glow radius to prevent overflow
            let glow_radius = glow_radius.min(500);
            
            // Skip pixels for distant layers (performance optimization)
            let pixel_skip = if layer > 3 { 2 } else { 1 };
            
            let mut y = -glow_radius;
            while y <= glow_radius {
                let mut x = -glow_radius;
                while x <= glow_radius {
                    let dist_sq = x * x + y * y;
                    let radius_sq = glow_radius * glow_radius;
                    
                    if dist_sq <= radius_sq {
                        let px = center_x + x;
                        let py = center_y + y;
                        
                        if px >= 0 && px < framebuffer.width as i32 && py >= 0 && py < framebuffer.height as i32 {
                            // Calculate distance factor for smooth gradient (reducido de 180 a 100)
                            let dist_factor = 1.0 - (dist_sq as f32 / radius_sq as f32).sqrt();
                            let glow_strength = (dist_factor * layer_intensity * 100.0) as u16;
                            
                            let current_color = framebuffer.get_pixel(px as u32, py as u32);
                            
                            // Enhanced glow with warmer tones but less intense
                            let glow_color = Color::new(
                                ((current_color.r as u16 + glow_strength).min(255)) as u8,
                                ((current_color.g as u16 + (glow_strength * 3 / 4)).min(255)) as u8,
                                ((current_color.b as u16 + (glow_strength / 6)).min(255)) as u8,
                                255
                            );
                            
                            framebuffer.set_pixel_color(px as u32, py as u32, glow_color);
                        }
                    }
                    x += pixel_skip;
                }
                y += pixel_skip;
            }
        }
    }

    fn render_obj_model_to_framebuffer(
        &self,
        framebuffer: &mut Framebuffer,
        model: &ObjModel,
        body: &CelestialBody,
        view_matrix: &Matrix,
    ) {
        // Create transformation matrices
        let scale_matrix = create_scale_matrix(body.radius, body.radius, body.radius);
        let translation_matrix = create_translation_matrix(body.position.x, body.position.y, body.position.z);
        
        // Combine transformations: Translation * Scale
        let model_matrix = translation_matrix.multiply(&scale_matrix);
        
        // Get base color for the body
        let base_color = body.color;

        // Calculate distance for LOD (Performance optimization)
        let distance_to_camera = vector_length(body.position);
        let apparent_radius = (body.radius * self.screen_width) / (distance_to_camera * 2.0);
        
        // Skip rendering if too small (ultra performance optimization)
        if apparent_radius < 0.3 {
            return;
        }
        
        // Aggressive adaptive LOD system for maximum performance
        let face_skip = if apparent_radius < 3.0 { 
            8  // Very distant: render 1/8 of faces (massive speedup)
        } else if apparent_radius < 8.0 { 
            4  // Distant: render 1/4 of faces
        } else if apparent_radius < 20.0 {
            2  // Medium: render 1/2 of faces
        } else { 
            1  // Close: render all faces
        };

        // Render each triangle from the model
        for (face_idx, face) in model.faces.iter().enumerate() {
            // LOD: skip faces based on distance (major performance boost)
            if face_skip > 1 && face_idx % face_skip != 0 {
                continue;
            }
            
            if face.vertices.len() < 3 {
                continue;
            }

            // Get first triangle (triangulate if needed)
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

                // Transform vertices
                let v0_world = model_matrix.transform_point(v0);
                let v1_world = model_matrix.transform_point(v1);
                let v2_world = model_matrix.transform_point(v2);

                // Transform to screen space
                let v0_screen = self.world_to_screen(v0_world, view_matrix);
                let v1_screen = self.world_to_screen(v1_world, view_matrix);
                let v2_screen = self.world_to_screen(v2_world, view_matrix);

                // Simple backface culling - check if triangle is facing away
                let edge1_x = v1_screen.x - v0_screen.x;
                let edge1_y = v1_screen.y - v0_screen.y;
                let edge2_x = v2_screen.x - v0_screen.x;
                let edge2_y = v2_screen.y - v0_screen.y;
                let cross_z = edge1_x * edge2_y - edge1_y * edge2_x;
                
                // Skip back-facing triangles
                if cross_z <= 0.0 {
                    continue;
                }

                // Calculate normal for lighting
                let edge1 = Vector3::new(v1_world.x - v0_world.x, v1_world.y - v0_world.y, v1_world.z - v0_world.z);
                let edge2 = Vector3::new(v2_world.x - v0_world.x, v2_world.y - v0_world.y, v2_world.z - v0_world.z);
                let normal = normalize_vector(Vector3::new(
                    edge1.y * edge2.z - edge1.z * edge2.y,
                    edge1.z * edge2.x - edge1.x * edge2.z,
                    edge1.x * edge2.y - edge1.y * edge2.x,
                ));

                // Calculate lighting
                let light_intensity = self.calculate_light_intensity(normal);
                let lit_color = self.apply_lighting(base_color, light_intensity);

                // Rasterize triangle
                self.rasterize_triangle_to_framebuffer(
                    framebuffer,
                    v0_screen,
                    v1_screen,
                    v2_screen,
                    lit_color,
                );
            }
        }
    }

    fn render_simple_sphere_to_framebuffer(
        &self,
        framebuffer: &mut Framebuffer,
        body: &CelestialBody,
        view_matrix: &Matrix,
    ) {
        let screen_pos = self.world_to_screen(body.position, view_matrix);
        let distance = vector_length(body.position);
        let apparent_radius = (body.radius * self.screen_width) / (distance * 2.0);
        
        let center_x = screen_pos.x as i32;
        let center_y = screen_pos.y as i32;
        let radius_i = apparent_radius as i32;

        // Draw filled circle using Midpoint Circle Algorithm
        for y in -radius_i..=radius_i {
            for x in -radius_i..=radius_i {
                if x * x + y * y <= radius_i * radius_i {
                    let px = center_x + x;
                    let py = center_y + y;
                    if px >= 0 && px < framebuffer.width as i32 && py >= 0 && py < framebuffer.height as i32 {
                        framebuffer.set_pixel_color(px as u32, py as u32, body.color);
                    }
                }
            }
        }
    }

    fn rasterize_triangle_to_framebuffer(
        &self,
        framebuffer: &mut Framebuffer,
        v0: Vector3,
        v1: Vector3,
        v2: Vector3,
        color: Color,
    ) {
        // Bounding box
        let min_x = v0.x.min(v1.x).min(v2.x).max(0.0) as i32;
        let max_x = v0.x.max(v1.x).max(v2.x).min(framebuffer.width as f32 - 1.0) as i32;
        let min_y = v0.y.min(v1.y).min(v2.y).max(0.0) as i32;
        let max_y = v0.y.max(v1.y).max(v2.y).min(framebuffer.height as f32 - 1.0) as i32;

        // Rasterize using barycentric coordinates
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let p = Vector3::new(x as f32 + 0.5, y as f32 + 0.5, 0.0);
                
                if self.is_inside_triangle(p, v0, v1, v2) {
                    framebuffer.set_pixel_color(x as u32, y as u32, color);
                }
            }
        }
    }

    fn is_inside_triangle(&self, p: Vector3, v0: Vector3, v1: Vector3, v2: Vector3) -> bool {
        let edge0 = (v1.x - v0.x) * (p.y - v0.y) - (v1.y - v0.y) * (p.x - v0.x);
        let edge1 = (v2.x - v1.x) * (p.y - v1.y) - (v2.y - v1.y) * (p.x - v1.x);
        let edge2 = (v0.x - v2.x) * (p.y - v2.y) - (v0.y - v2.y) * (p.x - v2.x);

        (edge0 >= 0.0 && edge1 >= 0.0 && edge2 >= 0.0) || (edge0 <= 0.0 && edge1 <= 0.0 && edge2 <= 0.0)
    }

    fn calculate_light_intensity(&self, normal: Vector3) -> f32 {
        let dot_product = -(normal.x * self.light_direction.x +
                           normal.y * self.light_direction.y +
                           normal.z * self.light_direction.z);
        let diffuse = dot_product.max(0.0);
        self.ambient_light + (1.0 - self.ambient_light) * diffuse
    }

    fn apply_lighting(&self, color: Color, intensity: f32) -> Color {
        Color::new(
            (color.r as f32 * intensity).min(255.0) as u8,
            (color.g as f32 * intensity).min(255.0) as u8,
            (color.b as f32 * intensity).min(255.0) as u8,
            color.a,
        )
    }

    fn render_orbits_to_framebuffer(
        &self,
        framebuffer: &mut Framebuffer,
        solar_system: &SolarSystem,
        view_matrix: &Matrix,
    ) {
        for planet in &solar_system.planets {
            let radius = planet.orbital_radius;
            let segments = 120; // Reducido de 150 a 120 para mejor performance
            
            // Enhanced color with optimized rendering
            let base_intensity = 0.5; // Reducido para menos procesamiento
            let orbit_color = Color::new(
                (planet.color.r as f32 * base_intensity) as u8,
                (planet.color.g as f32 * base_intensity) as u8,
                (planet.color.b as f32 * base_intensity) as u8,
                160 // Reducida opacidad para mejor blend
            );
            
            // Dimmer inner glow (only every 4th segment for performance)
            let glow_color = Color::new(
                (planet.color.r as f32 * 0.25) as u8,
                (planet.color.g as f32 * 0.25) as u8,
                (planet.color.b as f32 * 0.25) as u8,
                60 // Reducido para menos blend overhead
            );

            for i in 0..segments {
                let angle1 = (i as f32 / segments as f32) * 2.0 * PI;
                let angle2 = ((i + 1) as f32 / segments as f32) * 2.0 * PI;

                let x1 = radius * angle1.cos();
                let z1 = radius * angle1.sin();
                let p1 = Vector3::new(x1, 0.0, z1);

                let x2 = radius * angle2.cos();
                let z2 = radius * angle2.sin();
                let p2 = Vector3::new(x2, 0.0, z2);

                let screen1 = self.world_to_screen(p1, view_matrix);
                let screen2 = self.world_to_screen(p2, view_matrix);

                // Only draw if both points are on screen or close (tighter bounds)
                if (screen1.x >= -30.0 && screen1.x < self.screen_width + 30.0 &&
                    screen1.y >= -30.0 && screen1.y < self.screen_height + 30.0) ||
                   (screen2.x >= -30.0 && screen2.x < self.screen_width + 30.0 &&
                    screen2.y >= -30.0 && screen2.y < self.screen_height + 30.0) {
                    
                    // Draw main orbit line
                    framebuffer.set_current_color(orbit_color);
                    framebuffer.draw_line(
                        screen1.x as i32, screen1.y as i32,
                        screen2.x as i32, screen2.y as i32
                    );
                    
                    // Add subtle glow effect (every 5th segment for better performance)
                    if i % 5 == 0 {
                        // Draw glow line slightly offset for thickness
                        let offset_screen1_y = (screen1.y + 1.0) as i32;
                        let offset_screen2_y = (screen2.y + 1.0) as i32;
                        
                        framebuffer.set_current_color(glow_color);
                        framebuffer.draw_line(
                            screen1.x as i32, offset_screen1_y,
                            screen2.x as i32, offset_screen2_y
                        );
                    }
                }
            }
        }
    }
}
