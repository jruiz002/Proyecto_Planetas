#![allow(dead_code)]
use raylib::prelude::*;
use std::f32::consts::PI;

// Helper function to calculate vector length
fn vector_length(v: Vector3) -> f32 {
    (v.x * v.x + v.y * v.y + v.z * v.z).sqrt()
}

#[derive(Debug, Clone, Copy)]
pub enum CelestialBodyType {
    Star,
    Planet,
    Moon,
}

#[derive(Debug, Clone)]
pub struct CelestialBody {
    pub body_type: CelestialBodyType,
    pub position: Vector3,
    pub radius: f32,
    pub color: Color,
    pub rotation_angle: f32,
    pub rotation_speed: f32,
    
    // Orbital parameters
    pub orbital_center: Vector3,
    pub orbital_radius: f32,
    pub orbital_angle: f32,
    pub orbital_speed: f32,
    pub orbital_inclination: f32,
    
    // Visual properties
    pub name: String,
    pub has_rings: bool,
    pub ring_inner_radius: f32,
    pub ring_outer_radius: f32,
    pub ring_color: Color,
    
    // Moons (for planets)
    pub moons: Vec<CelestialBody>,
}

impl CelestialBody {
    pub fn new_star(
        name: String,
        position: Vector3,
        radius: f32,
        color: Color,
        rotation_speed: f32,
    ) -> Self {
        CelestialBody {
            body_type: CelestialBodyType::Star,
            position,
            radius,
            color,
            rotation_angle: 0.0,
            rotation_speed,
            orbital_center: Vector3::new(0.0, 0.0, 0.0),
            orbital_radius: 0.0,
            orbital_angle: 0.0,
            orbital_speed: 0.0,
            orbital_inclination: 0.0,
            name,
            has_rings: false,
            ring_inner_radius: 0.0,
            ring_outer_radius: 0.0,
            ring_color: Color::WHITE,
            moons: Vec::new(),
        }
    }

    pub fn new_planet(
        name: String,
        orbital_center: Vector3,
        orbital_radius: f32,
        orbital_speed: f32,
        radius: f32,
        color: Color,
        rotation_speed: f32,
        orbital_inclination: f32,
    ) -> Self {
        // Set a fixed position based on initial angle (no random movement)
        let orbital_angle = 0.0; // Fixed starting angle
        let position = Vector3::new(
            orbital_center.x + orbital_radius,
            orbital_center.y,
            orbital_center.z,
        );

        CelestialBody {
            body_type: CelestialBodyType::Planet,
            position,
            radius,
            color,
            rotation_angle: 0.0,
            rotation_speed,
            orbital_center,
            orbital_radius,
            orbital_angle,
            orbital_speed,
            orbital_inclination,
            name,
            has_rings: false,
            ring_inner_radius: 0.0,
            ring_outer_radius: 0.0,
            ring_color: Color::WHITE,
            moons: Vec::new(),
        }
    }

    pub fn new_moon(
        name: String,
        orbital_center: Vector3,
        orbital_radius: f32,
        orbital_speed: f32,
        radius: f32,
        color: Color,
        rotation_speed: f32,
    ) -> Self {
        // Set a fixed position for moon (no random movement)
        let orbital_angle = PI / 4.0; // Fixed 45 degree angle
        let position = Vector3::new(
            orbital_center.x + orbital_radius * orbital_angle.cos(),
            orbital_center.y,
            orbital_center.z + orbital_radius * orbital_angle.sin(),
        );

        CelestialBody {
            body_type: CelestialBodyType::Moon,
            position,
            radius,
            color,
            rotation_angle: 0.0,
            rotation_speed,
            orbital_center,
            orbital_radius,
            orbital_angle,
            orbital_speed,
            orbital_inclination: 0.0,
            name,
            has_rings: false,
            ring_inner_radius: 0.0,
            ring_outer_radius: 0.0,
            ring_color: Color::WHITE,
            moons: Vec::new(),
        }
    }

    pub fn add_rings(&mut self, inner_radius: f32, outer_radius: f32, color: Color) {
        self.has_rings = true;
        self.ring_inner_radius = inner_radius;
        self.ring_outer_radius = outer_radius;
        self.ring_color = color;
    }

    pub fn add_moon(&mut self, moon: CelestialBody) {
        self.moons.push(moon);
    }

    pub fn update(&mut self, dt: f32) {
        // Update rotation only (planets stay in fixed positions)
        self.rotation_angle += self.rotation_speed * dt;
        if self.rotation_angle > 2.0 * PI {
            self.rotation_angle -= 2.0 * PI;
        }

        // No orbital movement - planets remain stationary
        // Only update moons rotation (they also stay in fixed positions relative to their planet)
        for moon in &mut self.moons {
            moon.rotation_angle += moon.rotation_speed * dt;
            if moon.rotation_angle > 2.0 * PI {
                moon.rotation_angle -= 2.0 * PI;
            }
        }
    }

    pub fn get_orbit_points(&self, num_points: usize) -> Vec<Vector3> {
        let mut points = Vec::new();
        
        if self.orbital_radius > 0.0 {
            for i in 0..num_points {
                let angle = (i as f32 / num_points as f32) * 2.0 * PI;
                let point = Vector3::new(
                    self.orbital_center.x + self.orbital_radius * angle.cos(),
                    self.orbital_center.y + self.orbital_radius * self.orbital_inclination.sin() * angle.sin(),
                    self.orbital_center.z + self.orbital_radius * angle.sin(),
                );
                points.push(point);
            }
        }
        
        points
    }

    pub fn get_distance_from_camera(&self, camera_pos: Vector3) -> f32 {
        vector_length(self.position - camera_pos)
    }

    pub fn is_visible_from_camera(&self, camera_pos: Vector3, max_distance: f32) -> bool {
        self.get_distance_from_camera(camera_pos) <= max_distance
    }

    // Generate vertices for a sphere (simplified icosphere)
    pub fn get_sphere_vertices(&self, detail_level: u32) -> Vec<Vector3> {
        let mut vertices = Vec::new();
        
        // Simple sphere generation using latitude/longitude
        let lat_bands = detail_level + 10;
        let lon_bands = detail_level + 10;
        
        for lat in 0..=lat_bands {
            let theta = (lat as f32 / lat_bands as f32) * PI;
            let sin_theta = theta.sin();
            let cos_theta = theta.cos();
            
            for lon in 0..=lon_bands {
                let phi = (lon as f32 / lon_bands as f32) * 2.0 * PI;
                let sin_phi = phi.sin();
                let cos_phi = phi.cos();
                
                let x = cos_phi * sin_theta;
                let y = cos_theta;
                let z = sin_phi * sin_theta;
                
                vertices.push(Vector3::new(
                    self.position.x + x * self.radius,
                    self.position.y + y * self.radius,
                    self.position.z + z * self.radius,
                ));
            }
        }
        
        vertices
    }

    // Get triangles for rendering
    pub fn get_sphere_triangles(&self, detail_level: u32) -> Vec<[Vector3; 3]> {
        let mut triangles = Vec::new();
        let lat_bands = detail_level + 10;
        let lon_bands = detail_level + 10;
        
        for lat in 0..lat_bands {
            for lon in 0..lon_bands {
                let first = lat * (lon_bands + 1) + lon;
                let second = first + lon_bands + 1;
                
                // First triangle
                let v1 = self.get_sphere_vertex(lat, lon, lat_bands, lon_bands);
                let v2 = self.get_sphere_vertex(lat + 1, lon, lat_bands, lon_bands);
                let v3 = self.get_sphere_vertex(lat, lon + 1, lat_bands, lon_bands);
                triangles.push([v1, v2, v3]);
                
                // Second triangle
                let v4 = self.get_sphere_vertex(lat + 1, lon, lat_bands, lon_bands);
                let v5 = self.get_sphere_vertex(lat + 1, lon + 1, lat_bands, lon_bands);
                let v6 = self.get_sphere_vertex(lat, lon + 1, lat_bands, lon_bands);
                triangles.push([v4, v5, v6]);
            }
        }
        
        triangles
    }
    
    fn get_sphere_vertex(&self, lat: u32, lon: u32, lat_bands: u32, lon_bands: u32) -> Vector3 {
        let theta = (lat as f32 / lat_bands as f32) * PI;
        let phi = (lon as f32 / lon_bands as f32) * 2.0 * PI;
        
        let sin_theta = theta.sin();
        let cos_theta = theta.cos();
        let sin_phi = phi.sin();
        let cos_phi = phi.cos();
        
        let x = cos_phi * sin_theta;
        let y = cos_theta;
        let z = sin_phi * sin_theta;
        
        Vector3::new(
            self.position.x + x * self.radius,
            self.position.y + y * self.radius,
            self.position.z + z * self.radius,
        )
    }
}