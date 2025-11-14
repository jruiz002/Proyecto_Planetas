#![allow(dead_code)]
use raylib::prelude::*;
use crate::celestial_body::CelestialBody;
use std::f32::consts::PI;

pub struct SolarSystem {
    pub star: CelestialBody,
    pub planets: Vec<CelestialBody>,
    pub time_scale: f32,
}

impl SolarSystem {
    pub fn new() -> Self {
        let mut solar_system = SolarSystem {
            star: CelestialBody::new_star(
                "Sol".to_string(),
                Vector3::new(0.0, 0.0, 0.0),
                15.0,
                Color::YELLOW,
                0.5,
            ),
            planets: Vec::new(),
            time_scale: 1.0,
        };

        // Create an imaginative solar system with diverse planets
        solar_system.create_planets();
        solar_system
    }

    fn create_planets(&mut self) {
        // Planet 1: Rocky inner planet (Mercury-like) - at 0 degrees
        let mut planet1 = CelestialBody::new_planet(
            "Pyrion".to_string(),
            Vector3::new(0.0, 0.0, 0.0),
            40.0,   // orbital radius
            2.0,    // orbital speed (not used in static mode)
            3.0,    // radius
            Color::new(210, 105, 30, 255), // Chocolate brown - more vibrant
            3.0,    // rotation speed
            0.1,    // orbital inclination
        );

        // Planet 2: Earth-like planet with moon - at 72 degrees
        let mut planet2 = CelestialBody::new_planet(
            "Verdania".to_string(),
            Vector3::new(0.0, 0.0, 0.0),
            70.0,   // orbital radius
            1.5,    // orbital speed
            5.0,    // radius
            Color::new(34, 139, 34, 255), // Forest green
            2.0,    // rotation speed
            0.05,   // orbital inclination
        );
        // Manually set position for planet2 at 72 degrees
        let angle2 = 72.0_f32.to_radians();
        planet2.position = Vector3::new(70.0 * angle2.cos(), 0.0, 70.0 * angle2.sin());

        // Add moon to planet2
        let moon1 = CelestialBody::new_moon(
            "Luna Verde".to_string(),
            planet2.position,
            12.0,   // orbital radius around planet
            8.0,    // orbital speed
            1.5,    // radius
            Color::new(192, 192, 192, 255), // Silver
            1.0,    // rotation speed
        );
        planet2.add_moon(moon1);

        // Planet 3: Gas giant with rings - at 144 degrees
        let mut planet3 = CelestialBody::new_planet(
            "Gigantus".to_string(),
            Vector3::new(0.0, 0.0, 0.0),
            120.0,  // orbital radius
            1.0,    // orbital speed
            8.0,    // radius
            Color::new(255, 200, 50, 255), // Bright gold
            1.5,    // rotation speed
            0.15,   // orbital inclination
        );
        // Manually set position for planet3 at 144 degrees
        let angle3 = 144.0_f32.to_radians();
        planet3.position = Vector3::new(120.0 * angle3.cos(), 0.0, 120.0 * angle3.sin());

        // Add rings to planet3
        planet3.add_rings(10.0, 15.0, Color::new(220, 220, 180, 120));

        // Add multiple moons to planet3
        let moon2 = CelestialBody::new_moon(
            "Titan Dorado".to_string(),
            planet3.position,
            20.0,   // orbital radius
            4.0,    // orbital speed
            2.0,    // radius
            Color::new(255, 140, 0, 255), // Dark orange
            2.0,    // rotation speed
        );
        planet3.add_moon(moon2);

        let moon3 = CelestialBody::new_moon(
            "Io Menor".to_string(),
            planet3.position,
            25.0,   // orbital radius
            3.0,    // orbital speed
            1.0,    // radius
            Color::new(220, 20, 60, 255), // Crimson
            3.0,    // rotation speed
        );
        planet3.add_moon(moon3);

        // Planet 4: Ice planet - at 216 degrees
        let mut planet4 = CelestialBody::new_planet(
            "Glacialis".to_string(),
            Vector3::new(0.0, 0.0, 0.0),
            180.0,  // orbital radius
            0.7,    // orbital speed
            6.0,    // radius
            Color::new(135, 206, 250, 255), // Light sky blue - icy look
            1.0,    // rotation speed
            0.2,    // orbital inclination
        );
        // Manually set position for planet4 at 216 degrees
        let angle4 = 216.0_f32.to_radians();
        planet4.position = Vector3::new(180.0 * angle4.cos(), 0.0, 180.0 * angle4.sin());

        // Planet 5: Distant dwarf planet - at 288 degrees
        let mut planet5 = CelestialBody::new_planet(
            "Plutonix".to_string(),
            Vector3::new(0.0, 0.0, 0.0),
            250.0,  // orbital radius
            0.4,    // orbital speed
            2.5,    // radius
            Color::new(186, 85, 211, 255), // Medium orchid purple
            0.8,    // rotation speed
            0.3,    // orbital inclination
        );
        // Manually set position for planet5 at 288 degrees
        let angle5 = 288.0_f32.to_radians();
        planet5.position = Vector3::new(250.0 * angle5.cos(), 0.0, 250.0 * angle5.sin());

        // Add planets to the system
        self.planets.push(planet1);
        self.planets.push(planet2);
        self.planets.push(planet3);
        self.planets.push(planet4);
        self.planets.push(planet5);
    }

    pub fn update(&mut self, dt: f32) {
        let scaled_dt = dt * self.time_scale;
        
        // Update star
        self.star.update(scaled_dt);
        
        // Update all planets
        for planet in &mut self.planets {
            planet.update(scaled_dt);
        }
    }

    pub fn get_all_bodies(&self) -> Vec<&CelestialBody> {
        let mut bodies = vec![&self.star];
        for planet in &self.planets {
            bodies.push(planet);
            for moon in &planet.moons {
                bodies.push(moon);
            }
        }
        bodies
    }

    pub fn get_all_bodies_mut(&mut self) -> Vec<&mut CelestialBody> {
        let mut bodies = vec![&mut self.star];
        for planet in &mut self.planets {
            bodies.push(planet);
            // Note: Can't easily return mutable references to moons due to borrowing rules
            // Moons are updated through their parent planets
        }
        bodies
    }

    pub fn get_planet_by_index(&self, index: usize) -> Option<&CelestialBody> {
        self.planets.get(index)
    }

    pub fn set_time_scale(&mut self, scale: f32) {
        self.time_scale = scale.max(0.1).min(10.0); // Clamp between 0.1x and 10x
    }

    pub fn get_system_bounds(&self) -> (Vector3, Vector3) {
        let mut min_bounds = Vector3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);
        let mut max_bounds = Vector3::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY);

        // Check star
        let star_bounds = self.star.radius;
        min_bounds.x = min_bounds.x.min(self.star.position.x - star_bounds);
        min_bounds.y = min_bounds.y.min(self.star.position.y - star_bounds);
        min_bounds.z = min_bounds.z.min(self.star.position.z - star_bounds);
        max_bounds.x = max_bounds.x.max(self.star.position.x + star_bounds);
        max_bounds.y = max_bounds.y.max(self.star.position.y + star_bounds);
        max_bounds.z = max_bounds.z.max(self.star.position.z + star_bounds);

        // Check planets and their orbits
        for planet in &self.planets {
            let orbit_bounds = planet.orbital_radius + planet.radius;
            min_bounds.x = min_bounds.x.min(planet.orbital_center.x - orbit_bounds);
            min_bounds.y = min_bounds.y.min(planet.orbital_center.y - orbit_bounds);
            min_bounds.z = min_bounds.z.min(planet.orbital_center.z - orbit_bounds);
            max_bounds.x = max_bounds.x.max(planet.orbital_center.x + orbit_bounds);
            max_bounds.y = max_bounds.y.max(planet.orbital_center.y + orbit_bounds);
            max_bounds.z = max_bounds.z.max(planet.orbital_center.z + orbit_bounds);
        }

        (min_bounds, max_bounds)
    }

    pub fn get_total_body_count(&self) -> usize {
        let mut count = 1; // Star
        count += self.planets.len();
        for planet in &self.planets {
            count += planet.moons.len();
        }
        count
    }

    // Get bodies sorted by distance from camera (for proper rendering order)
    pub fn get_bodies_sorted_by_distance(&self, camera_pos: Vector3) -> Vec<&CelestialBody> {
        let mut bodies = self.get_all_bodies();
        bodies.sort_by(|a, b| {
            let dist_a = a.get_distance_from_camera(camera_pos);
            let dist_b = b.get_distance_from_camera(camera_pos);
            dist_b.partial_cmp(&dist_a).unwrap_or(std::cmp::Ordering::Equal)
        });
        bodies
    }

    // Find the closest body to a given position
    pub fn find_closest_body(&self, position: Vector3) -> Option<&CelestialBody> {
        let bodies = self.get_all_bodies();
        bodies.iter()
            .min_by(|a, b| {
                let dist_a = a.get_distance_from_camera(position);
                let dist_b = b.get_distance_from_camera(position);
                dist_a.partial_cmp(&dist_b).unwrap_or(std::cmp::Ordering::Equal)
            })
            .copied()
    }
}