mod matrix;
mod camera;
mod celestial_body;
mod renderer;
mod solar_system;
mod framebuffer;
mod obj_loader;
mod vertex_shader;
mod primitive_assembly;
mod rasterizer;
mod fragment_shader;

use raylib::prelude::*;

use crate::camera::Camera;
use crate::celestial_body::CelestialBody;
use crate::solar_system::SolarSystem;
use crate::renderer::Renderer;
use crate::framebuffer::Framebuffer;

const SCREEN_WIDTH: i32 = 1200;
const SCREEN_HEIGHT: i32 = 800;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Solar System Simulator")
        .build();

    rl.set_target_fps(60);

    // Initialize framebuffer
    let mut framebuffer = Framebuffer::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32);

    // Initialize camera
    let mut camera = Camera::new(
        Vector3::new(0.0, 50.0, 100.0),  // eye position
        Vector3::new(0.0, 0.0, 0.0),     // target
        Vector3::new(0.0, 1.0, 0.0),     // up vector
    );

    // Initialize solar system
    let mut solar_system = SolarSystem::new();
    
    // Initialize renderer
    let mut renderer = Renderer::new(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32);

    let mut time = 0.0f32;
    let mut show_orbits = true;
    let mut warp_mode = false;
    let mut selected_planet = 0;
    let mut follow_planet: Option<usize> = None;

    while !rl.window_should_close() {
        let dt = rl.get_frame_time();
        time += dt;

        // Handle input
        handle_input(&mut rl, &mut camera, &mut show_orbits, 
                    &mut warp_mode, &mut selected_planet, &mut follow_planet, &solar_system, dt);

        // Update solar system
        solar_system.update(dt);

        if let Some(i) = follow_planet {
            if solar_system.planets.len() > i {
                camera.set_target(solar_system.planets[i].position);
            } else {
                follow_planet = None;
            }
        }
        
        // Update camera
        camera.update(dt);

        // Prevent camera collision with celestial bodies
        let all_bodies: Vec<CelestialBody> = solar_system.get_all_bodies()
            .into_iter()
            .cloned()
            .collect();
        camera.avoid_collision(&all_bodies);
        
        // OPTIMIZACIÓN: Aplicar límites de distancia mínima
        // Evita que la cámara se acerque tanto que genere millones de fragmentos
        // Esto previene caídas de FPS cuando hay planetas grandes muy cerca
        camera.enforce_minimum_distance(&all_bodies);

        // Clear framebuffer
        framebuffer.clear();

        // Render to framebuffer
        renderer.render_solar_system_to_framebuffer(&mut framebuffer, &solar_system, &camera, show_orbits);

        // Swap buffers (convert framebuffer to texture)
        framebuffer.swap_buffers(&mut rl, &thread);

        // Draw to screen
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        // Draw framebuffer to screen
        framebuffer.draw_to_screen(&mut d);

        // Render UI on top
        render_ui(&mut d, &camera, show_orbits, warp_mode, selected_planet, &all_bodies);
    }
}

fn handle_input(
    rl: &mut RaylibHandle,
    camera: &mut Camera,
    show_orbits: &mut bool,
    warp_mode: &mut bool,
    selected_planet: &mut usize,
    follow_planet: &mut Option<usize>,
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
            *follow_planet = Some(0);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_TWO) && solar_system.planets.len() > 1 {
            camera.instant_warp_to_planet(&solar_system.planets[1]);
            *follow_planet = Some(1);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_THREE) && solar_system.planets.len() > 2 {
            camera.instant_warp_to_planet(&solar_system.planets[2]);
            *follow_planet = Some(2);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_FOUR) && solar_system.planets.len() > 3 {
            camera.instant_warp_to_planet(&solar_system.planets[3]);
            *follow_planet = Some(3);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_FIVE) && solar_system.planets.len() > 4 {
            camera.instant_warp_to_planet(&solar_system.planets[4]);
            *follow_planet = Some(4);
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
    all_bodies: &[CelestialBody],
) {
    let y_offset = 10;
    let mut current_y = y_offset;

    // Title with shadow effect
    d.draw_text("Solar System Simulator", 12, current_y + 2, 24, Color::new(0, 0, 0, 100));
    d.draw_text("Solar System Simulator", 10, current_y, 24, Color::new(255, 215, 0, 255));
    current_y += 35;

    // Controls section
    d.draw_rectangle(8, current_y - 2, 280, 140, Color::new(0, 0, 0, 150));
    d.draw_rectangle_lines(8, current_y - 2, 280, 140, Color::new(100, 100, 255, 200));
    
    d.draw_text("CONTROLS:", 15, current_y, 16, Color::new(100, 200, 255, 255));
    current_y += 20;
    
    d.draw_text("Mouse Drag: Rotate Camera", 15, current_y, 12, Color::WHITE);
    current_y += 16;
    d.draw_text("Mouse Wheel: Zoom", 15, current_y, 12, Color::WHITE);
    current_y += 16;
    d.draw_text("WASD/QE: Move Camera", 15, current_y, 12, Color::WHITE);
    current_y += 16;
    d.draw_text("O: Toggle Orbits", 15, current_y, 12, Color::WHITE);
    current_y += 16;
    d.draw_text("W: Toggle Warp Mode", 15, current_y, 12, Color::WHITE);
    current_y += 16;
    d.draw_text("1-5: Warp to Planet", 15, current_y, 12, Color::WHITE);
    current_y += 16;
    d.draw_text("Shift+1-5: Follow Planet", 15, current_y, 12, Color::WHITE);
    current_y += 16;
    d.draw_text("0: Warp to Sun", 15, current_y, 12, Color::WHITE);
    current_y += 20;

    // Status section
    d.draw_rectangle(8, current_y - 2, 200, 65, Color::new(0, 0, 0, 150));
    d.draw_rectangle_lines(8, current_y - 2, 200, 65, Color::new(100, 255, 100, 200));
    
    d.draw_text("STATUS:", 15, current_y, 16, Color::new(100, 255, 100, 255));
    current_y += 20;

    let orbits_text = if show_orbits { "ON" } else { "OFF" };
    let orbits_color = if show_orbits { Color::GREEN } else { Color::RED };
    d.draw_text(&format!("Orbits: {}", orbits_text), 15, current_y, 14, orbits_color);
    current_y += 18;

    let warp_text = if warp_mode { "ENABLED" } else { "DISABLED" };
    let warp_color = if warp_mode { Color::YELLOW } else { Color::GRAY };
    d.draw_text(&format!("Warp Mode: {}", warp_text), 15, current_y, 14, warp_color);
    current_y += 25;

    // Show collision status
    if let Some(collision_info) = camera.get_collision_status(all_bodies) {
        d.draw_rectangle(8, current_y - 2, 280, 25, Color::new(255, 100, 0, 150));
        d.draw_text(&format!("⚠ {}", collision_info), 15, current_y, 14, Color::ORANGE);
        current_y += 30;
    }

    // Warping indicator
    if camera.is_warping {
        let warp_y = 300;
        d.draw_rectangle(d.get_screen_width() / 2 - 150, warp_y - 10, 300, 60, Color::new(0, 0, 0, 200));
        d.draw_rectangle_lines(d.get_screen_width() / 2 - 150, warp_y - 10, 300, 60, Color::SKYBLUE);
        
        d.draw_text("WARPING...", d.get_screen_width() / 2 - 70, warp_y, 24, Color::SKYBLUE);
        
        let progress_text = format!("Progress: {:.1}%", camera.warp_progress * 100.0);
        d.draw_text(&progress_text, d.get_screen_width() / 2 - 60, warp_y + 25, 16, Color::WHITE);
        
        // Progress bar
        let bar_width = 250;
        let bar_x = d.get_screen_width() / 2 - bar_width / 2;
        d.draw_rectangle(bar_x, warp_y + 45, bar_width, 8, Color::DARKGRAY);
        d.draw_rectangle(bar_x, warp_y + 45, (bar_width as f32 * camera.warp_progress) as i32, 8, Color::SKYBLUE);
    }

    // Planet info in bottom right
    let info_x = d.get_screen_width() - 220;
    let info_y = d.get_screen_height() - 150;
    
    d.draw_rectangle(info_x - 5, info_y - 5, 210, 145, Color::new(0, 0, 0, 150));
    d.draw_rectangle_lines(info_x - 5, info_y - 5, 210, 145, Color::new(255, 200, 100, 200));
    
    d.draw_text("SOLAR SYSTEM:", info_x, info_y, 14, Color::new(255, 200, 100, 255));
    
    let planet_names = ["Pyrion", "Verdania", "Gigantus", "Glacialis", "Plutonix"];
    for (i, name) in planet_names.iter().enumerate() {
        let y = info_y + 20 + (i as i32 * 16);
        d.draw_text(&format!("{}: {}", i + 1, name), info_x, y, 12, Color::WHITE);
    }
    
    d.draw_text("0: Sol (Sun)", info_x, info_y + 20 + (5 * 16), 12, Color::YELLOW);
    d.draw_text("9: System View", info_x, info_y + 20 + (6 * 16), 12, Color::SKYBLUE);
}
