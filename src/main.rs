use minifb::{Key, KeyRepeat, Window, WindowOptions};
use std::time::Instant;

mod framebuffer;
mod math;
mod camera;
mod scene;
mod models;
mod renderer;
mod effects;
mod physics;
mod input;

use framebuffer::Framebuffer;
use math::{Vec3, Mat4, deg_to_rad, create_transform_matrix};
use camera::freecam::FreeCamera;
use scene::solar_system::SolarSystem;
use models::{generate_sphere, load_obj, Mesh};
use renderer::pipeline::Pipeline;
use effects::skybox::Skybox;
use effects::warp::WarpEffect;
use physics::resolve_camera_collisions;
use input::InputController;

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;

// ========== CONFIG DE LA NAVE ==========
const SHIP_SCALE: f32 = 0.20;     
const SHIP_DISTANCE: f32 = 6.0;  
const SHIP_HEIGHT_OFFSET: f32 = -1.0; 
const SHIP_YAW_OFFSET: f32 = 0.0; 
const SHIP_PITCH_OFFSET: f32 = 0.0;   

fn main() {
    // -------------------------
    // Ventana
    // -------------------------
    let mut window = Window::new(
        "Sistema Solar - Software Renderer",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .expect("No se pudo crear la ventana");

    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);
    let mut pipeline = Pipeline::new(WIDTH, HEIGHT);

    // -------------------------
    // Cámara
    // -------------------------
    let mut camera = FreeCamera::new(Vec3::new(0.0, 5.0, 30.0));

    let mut input = InputController::new();

    // -------------------------
    // Objetos
    // -------------------------
    let skybox = Skybox::new(300);
    let sphere_mesh = generate_sphere(16, 16);
    let mut solar_system = SolarSystem::new(sphere_mesh);

    // Nave OBJ
    let ship_mesh = load_obj("assets/models/ship.obj", SHIP_SCALE);

    // Warp
    let mut warp = WarpEffect::new();

    // -------------------------
    // FPS
    // -------------------------
    let mut fps_timer = Instant::now();
    let mut fps_count = 0u64;

    let mut last_time = Instant::now();

    println!("===========================================");
    println!("  Sistema Solar - Software Renderer (Rust)");
    println!("===========================================");
    println!("Controles:");
    println!("  WASD - mover");
    println!("  SHIFT - rápido");
    println!("  Mouse - rotar cámara");
    println!("  SPACE - subir");
    println!("  CTRL - bajar");
    println!("  1/2/3 - warp a planetas");
    println!("  ESC - salir");
    println!("===========================================\n");

    while window.is_open() && !window.is_key_down(Key::Escape) {

        // =======================
        // DT
        // =======================
        let now = Instant::now();
        let dt = (now - last_time).as_secs_f32().min(0.05);
        last_time = now;

        // =======================
        // INPUT
        // =======================
        input.update(&window, &mut camera, dt);

        // Warp shortcuts
        if window.is_key_pressed(Key::Key1, KeyRepeat::No) {
            if let Some(p) = solar_system.planets.get(0) {
                warp.start(camera.position, p.position() + Vec3::new(0.0, 3.0, 12.0));
            }
        }
        if window.is_key_pressed(Key::Key2, KeyRepeat::No) {
            if let Some(p) = solar_system.planets.get(1) {
                warp.start(camera.position, p.position() + Vec3::new(0.0, 3.0, 12.0));
            }
        }
        if window.is_key_pressed(Key::Key3, KeyRepeat::No) {
            if let Some(p) = solar_system.planets.get(2) {
                warp.start(camera.position, p.position() + Vec3::new(0.0, 3.0, 12.0));
            }
        }

        warp.update(dt, &mut camera.position);
        solar_system.update(dt);

        // colisión cámara vs planetas
        resolve_camera_collisions(&solar_system, &mut camera.position, 1.5);

        // =======================
        // Matrices
        // =======================
        let view = camera.view_matrix();
        let aspect = WIDTH as f32 / HEIGHT as f32;
        let projection = Mat4::perspective(deg_to_rad(60.0), aspect, 0.1, 1000.0);

        // =======================
        // RENDER
        // =======================
        framebuffer.clear(0xFF000A0F);

        // skybox
        skybox.render(&mut framebuffer, view, projection);


        // sistema solar
        solar_system.render(&mut framebuffer, &mut pipeline, view, projection);

        // nave
        render_ship(
            &mut framebuffer,
            &mut pipeline,
            &camera,
            &ship_mesh,
            view,
            projection,
        );

        // =======================
        // Mostrar
        // =======================
        window
            .update_with_buffer(&framebuffer.buffer, WIDTH, HEIGHT)
            .unwrap();

        // =======================
        // FPS
        // =======================
        fps_count += 1;
        if fps_timer.elapsed().as_secs() >= 1 {
            let fps = fps_count as f32 / fps_timer.elapsed().as_secs_f32();
            window.set_title(&format!("Sistema Solar | FPS: {:.1}", fps));
            fps_count = 0;
            fps_timer = Instant::now();
        }
    }
}



// =========================================
// Render de nave (HUD 3D correcto)
// =========================================
fn render_ship(
    framebuffer: &mut Framebuffer,
    pipeline: &mut Pipeline,
    camera: &FreeCamera,
    ship_mesh: &Mesh,
    view: Mat4,
    projection: Mat4,
) {
    // Direcciones de la cámara
    let forward = camera.forward();
    let up = camera.up();

    // --------------------------
    // POSICIÓN FINAL DE LA NAVE
    // --------------------------
    let ship_pos =
        camera.position
        + forward * SHIP_DISTANCE
        + Vec3::new(0.0, SHIP_HEIGHT_OFFSET, 0.0);

    // --------------------------
    // ORIENTACIÓN REAL DE LA NAVE
    // Ajuste universal: rotar -90° en Y
    // para que el frente mire adelante
    // --------------------------
    let yaw = camera.yaw + SHIP_YAW_OFFSET - std::f32::consts::FRAC_PI_2;
    let pitch = camera.pitch + SHIP_PITCH_OFFSET;

    let ship_rot = Vec3::new(pitch, yaw, 0.0);

    // --------------------------
    // ESCALA FINAL
    // --------------------------
    let ship_scale = Vec3::new(SHIP_SCALE, SHIP_SCALE, SHIP_SCALE);

    // --------------------------
    // MATRIZ FINAL
    // --------------------------
    let model = create_transform_matrix(ship_pos, ship_rot, ship_scale);
    let mvp = projection * view * model;

    pipeline.set_mvp(mvp);
    pipeline.set_color(0xFFFFFFFF);

    pipeline.draw_mesh(
        framebuffer,
        &ship_mesh.vertices,
        &ship_mesh.faces,
    );
}
