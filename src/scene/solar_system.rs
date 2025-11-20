use crate::scene::celestial_body::CelestialBody;
use crate::renderer::pipeline::Pipeline;
use crate::framebuffer::Framebuffer;
use crate::math::{Mat4};
use crate::models::mesh::Mesh;

pub struct SolarSystem {
    pub sun: CelestialBody,
    pub planets: Vec<CelestialBody>,
    pub sphere_mesh: Mesh, // malla de esfera low-poly
}

impl SolarSystem {
    pub fn new(sphere_mesh: Mesh) -> Self {
        let sun = CelestialBody::new("Sun", 4.0, 0.0, 0.0, 0.3, 0xFFFFDD44);

        let mut planets = vec![
            CelestialBody::new("PlanetA", 1.5, 10.0, 0.4, 0.8, 0xFF44AAFF),
            CelestialBody::new("PlanetB", 1.0, 16.0, 0.3, 1.2, 0xFFFF8844),
            CelestialBody::new("PlanetC", 2.5, 24.0, 0.1, 0.4, 0xFF88FF44),
        ];

        Self {
            sun,
            planets,
            sphere_mesh,
        }
    }

    /// Actualiza todos los cuerpos del sistema solar
    pub fn update(&mut self, dt: f32) {
        self.sun.update(dt);
        for p in &mut self.planets {
            p.update(dt);
        }
    }

    /// Renderiza todos los cuerpos usando la malla de esfera
    pub fn render(&self, fb: &mut Framebuffer, pipeline: &mut Pipeline, view: Mat4, projection: Mat4) {
        
        // Render Sun
        self.draw_body(&self.sun, fb, pipeline, view, projection);

        // Render planets
        for planet in &self.planets {
            self.draw_body(planet, fb, pipeline, view, projection);
        }
    }

    fn draw_body(
        &self,
        body: &CelestialBody,
        fb: &mut Framebuffer,
        pipeline: &mut Pipeline,
        view: Mat4,
        projection: Mat4,
    ) {
        let model = body.model_matrix();
        let mvp = projection * view * model;

        pipeline.set_color(body.color);
        pipeline.set_mvp(mvp);

        pipeline.draw_mesh(fb, &self.sphere_mesh.vertices, &self.sphere_mesh.faces);
    }
}
