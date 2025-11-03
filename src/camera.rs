#![allow(dead_code)]
use raylib::prelude::*;
use crate::matrix::create_view_matrix;
use crate::celestial_body::CelestialBody;
use std::f32::consts::PI;

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

pub struct Camera {
    // Camera position/orientation
    pub eye: Vector3,        // Camera position
    pub target: Vector3,     // Point the camera is looking at
    pub up: Vector3,         // Up vector

    // Orbit camera parameters
    pub yaw: f32,            // Rotation around Y axis (left/right)
    pub pitch: f32,          // Rotation around X axis (up/down)
    pub distance: f32,       // Distance from target

    // Movement speed
    pub rotation_speed: f32,
    pub zoom_speed: f32,
    pub pan_speed: f32,
    pub move_speed: f32,

    // Warping animation
    pub is_warping: bool,
    pub warp_start_pos: Vector3,
    pub warp_target_pos: Vector3,
    pub warp_progress: f32,
    pub warp_duration: f32,
}

impl Camera {
    pub fn new(eye: Vector3, target: Vector3, up: Vector3) -> Self {
        let direction_vec = eye - target;
        let distance = vector_length(direction_vec);
        let direction = normalize_vector(direction_vec);
        
        // Calculate initial yaw and pitch from the direction vector
        let yaw = direction.z.atan2(direction.x);
        let pitch = direction.y.asin();

        Camera {
            eye,
            target,
            up,
            yaw,
            pitch: pitch.clamp(-PI/2.0 + 0.1, PI/2.0 - 0.1),
            distance,
            rotation_speed: 2.0,
            zoom_speed: 50.0,
            pan_speed: 30.0,
            move_speed: 100.0,
            is_warping: false,
            warp_start_pos: Vector3::zero(),
            warp_target_pos: Vector3::zero(),
            warp_progress: 0.0,
            warp_duration: 2.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        if self.is_warping {
            self.warp_progress += dt / self.warp_duration;
            
            if self.warp_progress >= 1.0 {
                self.warp_progress = 1.0;
                self.is_warping = false;
                self.eye = self.warp_target_pos;
            } else {
                // Smooth interpolation using ease-in-out
                let t = self.smooth_step(self.warp_progress);
                self.eye = Vector3::new(
                    self.warp_start_pos.x + (self.warp_target_pos.x - self.warp_start_pos.x) * t,
                    self.warp_start_pos.y + (self.warp_target_pos.y - self.warp_start_pos.y) * t,
                    self.warp_start_pos.z + (self.warp_target_pos.z - self.warp_start_pos.z) * t,
                );
            }
        }

        self.update_camera_position();
    }

    fn smooth_step(&self, t: f32) -> f32 {
        t * t * (3.0 - 2.0 * t)
    }

    fn update_camera_position(&mut self) {
        if !self.is_warping {
            // Calculate camera position based on spherical coordinates
            let x = self.target.x + self.distance * self.yaw.cos() * self.pitch.cos();
            let y = self.target.y + self.distance * self.pitch.sin();
            let z = self.target.z + self.distance * self.yaw.sin() * self.pitch.cos();
            
            self.eye = Vector3::new(x, y, z);
        }
    }

    pub fn rotate(&mut self, delta_x: f32, delta_y: f32) {
        if !self.is_warping {
            self.yaw -= delta_x * self.rotation_speed;
            self.pitch += delta_y * self.rotation_speed;
            
            // Clamp pitch to avoid gimbal lock
            self.pitch = self.pitch.clamp(-PI/2.0 + 0.1, PI/2.0 - 0.1);
        }
    }

    pub fn zoom(&mut self, delta: f32) {
        if !self.is_warping {
            self.distance -= delta * self.zoom_speed;
            self.distance = self.distance.clamp(5.0, 1000.0);
        }
    }

    pub fn pan(&mut self, delta_x: f32, delta_y: f32) {
        if !self.is_warping {
            let right = self.get_right_vector();
            let up = self.get_up_vector();
            
            let pan_amount = self.pan_speed * self.distance * 0.001;
            self.target = self.target + right * delta_x * pan_amount + up * delta_y * pan_amount;
        }
    }

    pub fn move_camera(&mut self, movement: Vector3, dt: f32) {
        if !self.is_warping {
            let forward = self.get_forward_vector();
            let right = self.get_right_vector();
            let up = self.get_up_vector();
            
            let move_amount = self.move_speed * dt;
            let movement_world = forward * movement.z * move_amount + 
                               right * movement.x * move_amount + 
                               up * movement.y * move_amount;
            
            self.target = self.target + movement_world;
        }
    }

    pub fn warp_to_planet(&mut self, planet: &CelestialBody) {
        if !self.is_warping {
            self.warp_start_pos = self.eye;
            
            // Position camera at a good distance from the planet
            let warp_distance = planet.radius * 5.0;
            let offset = Vector3::new(warp_distance, warp_distance * 0.5, warp_distance);
            self.warp_target_pos = planet.position + offset;
            
            self.target = planet.position;
            self.distance = warp_distance * 1.2;
            
            self.is_warping = true;
            self.warp_progress = 0.0;
        }
    }

    pub fn get_view_matrix(&self) -> crate::matrix::Matrix {
        create_view_matrix(self.eye, self.target, self.up)
    }

    pub fn get_forward_vector(&self) -> Vector3 {
        normalize_vector(self.target - self.eye)
    }

    pub fn get_right_vector(&self) -> Vector3 {
        let forward = self.get_forward_vector();
        let right_vec = Vector3::new(
            forward.y * self.up.z - forward.z * self.up.y,
            forward.z * self.up.x - forward.x * self.up.z,
            forward.x * self.up.y - forward.y * self.up.x,
        );
        normalize_vector(right_vec)
    }

    pub fn get_up_vector(&self) -> Vector3 {
        let forward = self.get_forward_vector();
        let right = self.get_right_vector();
        let up_vec = Vector3::new(
            right.y * forward.z - right.z * forward.y,
            right.z * forward.x - right.x * forward.z,
            right.x * forward.y - right.y * forward.x,
        );
        normalize_vector(up_vec)
    }

    pub fn get_position(&self) -> Vector3 {
        self.eye
    }

    pub fn get_target(&self) -> Vector3 {
        self.target
    }

    pub fn set_target(&mut self, new_target: Vector3) {
        if !self.is_warping {
            self.target = new_target;
        }
    }

    // Check if camera would collide with a celestial body
    pub fn check_collision(&self, body: &CelestialBody) -> bool {
        let distance_to_body = vector_length(self.eye - body.position);
        distance_to_body < body.radius * 1.5 // Add some buffer
    }

    // Prevent camera from going through celestial bodies
    pub fn avoid_collision(&mut self, bodies: &[CelestialBody]) {
        for body in bodies {
            if self.check_collision(body) {
                let direction = normalize_vector(self.eye - body.position);
                let safe_distance = body.radius * 1.5;
                self.eye = body.position + direction * safe_distance;
                
                // Update distance to maintain camera behavior
                self.distance = vector_length(self.eye - self.target);
            }
        }
    }
}