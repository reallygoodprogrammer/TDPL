//! # Particle
//!
//! Data structure and implementation for a single point
//! particle. Particles aren't intended to be created singularly
//! but rather used within a LinearParticles, PlanarParticles, or
//! SpatialParticles object.

use macroquad::color::Color;
use macroquad::math::Vec3;
use macroquad::prelude::draw_cube;
use std::time::{Duration, Instant};

/// Single Particle struct. Contains the `location`, `color`, and
/// `size` of the Particle.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Particle {
    location: Vec3,
    color: Color,
    size: f32,
    length: Duration,
    sloped: bool,
    start_time: Instant,
}

impl Particle {
    /// Instantiate a new Particle at `(x, y, z)` location
    /// with `(r, g, b, a)` color and `s` size, length `l`.
    pub fn new(
        (x, y, z): (f32, f32, f32),
        (r, g, b, a): (f32, f32, f32, f32),
        size: f32,
        l: f32,
        sloped: bool,
    ) -> Self {
        Particle {
            location: Vec3::new(x, y, z),
            color: Color::new(r, g, b, a),
            size,
            length: Duration::from_millis((l * 1000.) as u64),
            sloped,
            start_time: Instant::now(),
        }
    }

    /// Add the `x`, `y`, `z` argument values to the location of Particle.
    #[inline]
    pub fn add_location(mut self, x: f32, y: f32, z: f32) -> Self {
        self.location.x += x;
        self.location.y += y;
        self.location.z += z;
        self
    }

    /// Subtract the `x`, `y`, `z` argument values to the location of Particle.
    #[inline]
    pub fn sub_location(mut self, x: f32, y: f32, z: f32) -> Self {
        self.location.x -= x;
        self.location.y -= y;
        self.location.z -= z;
        self
    }

    /// Set the location of the particle to `x`, `y`, `z` argument.
    #[inline]
    pub fn set_location(&mut self, x: f32, y: f32, z: f32) {
        self.location = Vec3::new(x, y, z);
    }

    /// Set the color of the particle to `r`, `g`, `b`, `a` argument.
    #[inline]
    pub fn set_color(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.color = Color::new(r, g, b, a);
    }

    /// Set the size of the particle to `s` argument.
    #[inline]
    pub fn set_size(&mut self, s: f32) {
        self.size = s;
    }

    /// Draw the Particle within the macroquad world coords.
    #[inline]
    pub fn draw(&mut self) {
        draw_cube(self.location, Vec3::splat(self.size), None, self.color);
        if self.sloped {
            self.color.r /= 2.;
            self.color.g /= 2.;
            self.color.b /= 2.;
            self.color.a /= 2.;
        }
    }

    /// Check if the Particle has finished its lifetime
    #[inline]
    pub fn is_finished(&self) -> bool {
        self.start_time.elapsed() > self.length
    }

    /// Reset the ellapsed time for the Particle object
    pub fn reset(&mut self) {
        self.start_time = Instant::now();
    }
}

impl Default for Particle {
    fn default() -> Self {
        Particle::new((0., 0., 0.), (0., 0., 0., 1.), 0.01, 1., false)
    }
}
