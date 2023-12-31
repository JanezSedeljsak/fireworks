use crate::particle::Particle;
use nannou::prelude::*;
use nannou::color::Rgba;

const RGB_SIZE: usize = 255;
const BASE_ALPHA: f32 = 0.1;
const MAX_ALPHA: f32 = 0.8;

pub struct TrailParticle {
    position: Point2,
    color: Rgba,
}

impl TrailParticle {
    pub fn new(particle: &Particle) -> Self {
        TrailParticle {
            position: particle.position,
            color: particle.color,
        }
    }

    pub fn draw(&self, draw: &Draw, size: usize, ix: usize) {
        let new_alpha = BASE_ALPHA + scale(RGB_SIZE, ix) as f32 / RGB_SIZE as f32;
        let new_color = rgba(
            self.color.red,
            self.color.green,
            self.color.blue,
            f32::max(new_alpha, MAX_ALPHA)
        );
        draw.ellipse()
            .color(new_color)
            .x_y(self.position.x, self.position.y)
            .radius(scale(size, ix));
    }
}

fn scale(base_size: usize, minify_factor: usize) -> f32 {
    let factor = (base_size - minify_factor) as f32 / base_size as f32;
    base_size as f32 * 0.5 * (1.0 - factor)
}