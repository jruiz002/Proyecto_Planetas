mod matrix;
mod camera;
mod celestial_body;
mod renderer;
mod solar_system;

use raylib::prelude::*;

use crate::camera::Camera;
use crate::solar_system::SolarSystem;
use crate::renderer::Renderer;

const SCREEN_WIDTH: i32 = 1200;
const SCREEN_HEIGHT: i32 = 800;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Solar System Simulator")
        .build();

    rl.set_target_fps(60);

    // Initialize camera
    let mut camera = Camera::new(
        Vector3::new(0.0, 50.0, 100.0),  // eye position
        Vector3::new(0.0, 0.0, 0.0),     // target
        Vector3::new(0.0, 1.0, 0.0),     // up vector
    );

    // Initialize solar system
    let mut solar_system = SolarSystem::new();
    
    // Initialize renderer
    let renderer = Renderer::new(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32);

    let mut time = 0.0f32;
    let mut show_orbits = true;
    let mut warp_mode = false;
    let mut selected_planet = 0;

    while !rl.window_should_close() {
        let dt = rl.get_frame_time();
        time += dt;

        // Handle input
        handle_input(&mut rl, &mut camera, &mut show_orbits, 
                    &mut warp_mode, &mut selected_planet, &solar_system, dt);

        // Update solar system
        solar_system.update(dt);

        // Update camera
        camera.update(dt);

        // Render
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        // Render skybox (stars)
        renderer.render_skybox(&mut d, &camera);

        // Render solar system
        renderer.render_solar_system(&mut d, &solar_system, &camera, show_orbits);

        // Render UI
        render_ui(&mut d, &camera, show_orbits, warp_mode, selected_planet);
    }
}

fn handle_input(
    rl: &mut RaylibHandle,
    camera: &mut Camera,
    show_orbits: &mut bool,
    warp_mode: &mut bool,
    selected_planet: &mut usize,
    solar_system: &SolarSystem,
    dt: f32,
) {
    // Camera controls
    if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
        let mouse_delta = rl.get_mouse_delta();
        camera.rotate(mouse_delta.x * dt, mouse_delta.y * dt);
    }

    let wheel_move = rl.get_mouse_wheel_move();
    if wheel_move != 0.0 {
        camera.zoom(wheel_move * dt);
    }

    // Keyboard controls
    if rl.is_key_pressed(KeyboardKey::KEY_O) {
        *show_orbits = !*show_orbits;
    }

    if rl.is_key_pressed(KeyboardKey::KEY_W) {
        *warp_mode = !*warp_mode;
    }

    // Warp al sol (tecla 0)
    if rl.is_key_pressed(KeyboardKey::KEY_ZERO) {
        if *warp_mode {
            camera.warp_to_sun();
        }
    }

    // Vista general del sistema (tecla 9)
    if rl.is_key_pressed(KeyboardKey::KEY_NINE) {
        if *warp_mode {
            camera.warp_to_system_overview();
        }
    }

    // Planet selection for warping
    if rl.is_key_pressed(KeyboardKey::KEY_ONE) {
        *selected_planet = 0;
        if *warp_mode && solar_system.planets.len() > *selected_planet {
            camera.warp_to_planet(&solar_system.planets[*selected_planet]);
        }
    }
    if rl.is_key_pressed(KeyboardKey::KEY_TWO) && solar_system.planets.len() > 1 {
        *selected_planet = 1;
        if *warp_mode {
            camera.warp_to_planet(&solar_system.planets[*selected_planet]);
        }
    }
    if rl.is_key_pressed(KeyboardKey::KEY_THREE) && solar_system.planets.len() > 2 {
        *selected_planet = 2;
        if *warp_mode {
            camera.warp_to_planet(&solar_system.planets[*selected_planet]);
        }
    }
    if rl.is_key_pressed(KeyboardKey::KEY_FOUR) && solar_system.planets.len() > 3 {
        *selected_planet = 3;
        if *warp_mode {
            camera.warp_to_planet(&solar_system.planets[*selected_planet]);
        }
    }
    if rl.is_key_pressed(KeyboardKey::KEY_FIVE) && solar_system.planets.len() > 4 {
        *selected_planet = 4;
        if *warp_mode {
            camera.warp_to_planet(&solar_system.planets[*selected_planet]);
        }
    }

    // Warps instantaneos (sin animacion) con Shift
    if rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) || rl.is_key_down(KeyboardKey::KEY_RIGHT_SHIFT) {
        if rl.is_key_pressed(KeyboardKey::KEY_ONE) && solar_system.planets.len() > 0 {
            camera.instant_warp_to_planet(&solar_system.planets[0]);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_TWO) && solar_system.planets.len() > 1 {
            camera.instant_warp_to_planet(&solar_system.planets[1]);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_THREE) && solar_system.planets.len() > 2 {
            camera.instant_warp_to_planet(&solar_system.planets[2]);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_FOUR) && solar_system.planets.len() > 3 {
            camera.instant_warp_to_planet(&solar_system.planets[3]);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_FIVE) && solar_system.planets.len() > 4 {
            camera.instant_warp_to_planet(&solar_system.planets[4]);
        }
    }

    // WASD movement
    let mut movement = Vector3::zero();
    if rl.is_key_down(KeyboardKey::KEY_W) {
        movement.z -= 1.0;
    }
    if rl.is_key_down(KeyboardKey::KEY_S) {
        movement.z += 1.0;
    }
    if rl.is_key_down(KeyboardKey::KEY_A) {
        movement.x -= 1.0;
    }
    if rl.is_key_down(KeyboardKey::KEY_D) {
        movement.x += 1.0;
    }
    if rl.is_key_down(KeyboardKey::KEY_Q) {
        movement.y -= 1.0;
    }
    if rl.is_key_down(KeyboardKey::KEY_E) {
        movement.y += 1.0;
    }

    if movement.length() > 0.0 {
        camera.move_camera(movement, dt);
    }
}

fn render_ui(
    d: &mut RaylibDrawHandle,
    camera: &Camera,
    show_orbits: bool,
    warp_mode: bool,
    _selected_planet: usize,
) {
    let y_offset = 10;
    let mut current_y = y_offset;

    d.draw_text("Solar System Simulator", 10, current_y, 20, Color::WHITE);
    current_y += 30;

    // Solo mostrar información de estado esencial
    let orbits_text = if show_orbits { "ON" } else { "OFF" };
    d.draw_text(&format!("Orbits: {}", orbits_text), 10, current_y, 14, Color::WHITE);
    current_y += 20;

    let warp_text = if warp_mode { "ON" } else { "OFF" };
    //d.draw_text(&format!("Warp Mode: {}", warp_text), 10, current_y, 14, Color::WHITE);
    current_y += 20;

    if camera.is_warping {
        d.draw_text("WARPING...", 10, current_y, 16, Color::YELLOW);
        current_y += 20;
        d.draw_text(&format!("Progress: {:.1}%", camera.warp_progress * 100.0), 10, current_y, 14, Color::WHITE);
    }
}