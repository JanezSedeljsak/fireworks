use nannou::geom::Point2;
use nannou::color::Rgba;
use nannou::prelude::*;
use std::time::Instant;
use std::collections::LinkedList;
use crate::trail::TrailParticle;

const X_START: f32 = 500.0;
const Y_START: f32 = -500.0;

const RAND_EXPLOSION_START: f32 = 50.0;
const RAND_EXPLOSION_END: f32 = 300.0;

pub struct Particle {
    pub position: Point2,
    pub color: Rgba,
    is_from_left: bool,
    size: f32,
    direction_vec: Point2,
    particles: Option<Box<[Particle; 20]>>,
    exploded_at: Option<Instant>,
    trail: LinkedList<TrailParticle>,
}

impl Particle {

    pub fn new() -> Self {
        let random_position = pt2(random_range(-X_START, X_START), random_range(Y_START, Y_START + 100.0));
        let random_color = rgba(random_f32(), random_f32(), random_f32(), 1.0);
        let direction = pt2(random_range(-1.5, 1.5), random_range(0.5, 1.0) * 15.0);

        Particle {
            position: random_position,
            color: random_color,
            direction_vec: direction,
            is_from_left: random_position.x < 0.0,
            size: 5.0,
            particles: None,
            trail: LinkedList::new(),
            exploded_at: None
        }
    }
    
    pub fn update(&mut self) {
        match (self.exploded_at, self.particles.as_mut()) {
            (None, _) => {
                let inward_offset = f32::pow(self.direction_vec.y, 2.0) * random_f32() * 0.002;
                let adjusted_offset = if !self.is_from_left { -inward_offset } else { inward_offset };

                self.position.x += self.direction_vec.x;
                self.position.y += self.direction_vec.y;
                self.direction_vec.x += adjusted_offset;
                if self.get_should_explode() {
                    self.explode();
                }

                self.trail.push_back(TrailParticle::new(self));
                if self.trail.len() > 10 {
                    self.trail.pop_front();
                }
            }
            (_, Some(ref mut particles)) => {
                let speed_decrease = (1.0 + f32::min(self.exploded_at.unwrap().elapsed().as_secs() as f32, 3.0)) * 0.1;
                for particle in particles.iter_mut() {
                    particle.position.x += particle.direction_vec.x;
                    particle.position.y += particle.direction_vec.y;
                    particle.trail.push_back(TrailParticle::new(particle));
                    if particle.trail.len() > 2 {
                        particle.trail.pop_front();
                    }
                    
                    particle.direction_vec.y -= speed_decrease;
                    particle.direction_vec.x -= f32::pow(speed_decrease, 2);
                    let new_alpha = particle.color.alpha - (random_f32() * 0.05);
                    particle.color = rgba(
                        particle.color.red, 
                        particle.color.green, 
                        particle.color.blue, 
                        if new_alpha > 0.0 { new_alpha } else { 0.0 }
                    );
                }

                if self.is_dead() {
                    *self = Particle::new();
                }
            }
            _ => ()
        }            
    }

    pub fn draw(&self, draw: &Draw) {
        match (self.exploded_at, self.particles.as_ref()) {
            (None, _) => {
                for (ix, trail) in self.trail.iter().enumerate() {
                    trail.draw(draw, self.trail.len(), ix);
                }

                draw.ellipse()
                    .color(self.color)
                    .x_y(self.position.x, self.position.y)
                    .radius(self.size);
            }
            (_, Some(ref particles)) => {
                for particle in particles.iter() {
                    for (ix, trail) in particle.trail.iter().enumerate() {
                        trail.draw(draw, particle.trail.len(), ix);
                    }

                    draw.ellipse()
                        .color(particle.color)
                        .x_y(particle.position.x, particle.position.y)
                        .radius(particle.size);
                }
            }
            _ => ()
        }
    }

    fn get_should_explode(&self) -> bool {
        self.position.y > 50.0 && random_f32() < 0.05 || self.position.y > 300.0
    }

    fn is_dead(&self) -> bool {
        match self.exploded_at {
            None => self.position.y > RAND_EXPLOSION_START && random_f32() < 0.05 || self.position.y > RAND_EXPLOSION_END,
            _ => {
                let is_invisible = match self.particles {
                    None => false,
                    Some(ref particles) => particles.iter().all(|particle| particle.color.alpha <= 0.0)
                };
                let is_too_old = self.exploded_at.unwrap().elapsed().as_secs() > 5;
                is_invisible || is_too_old
            }
        }
    }

    fn explode(&mut self) {
        self.exploded_at = Some(Instant::now());
        let particles_vector: Vec<Particle> = (0..20).map(|_| {
            let random_position = pt2(self.position.x, self.position.y);
            let random_color = self.color;
            let direction = pt2(random_range(-1.0, 1.0) * 10.0, random_range(-1.0, 0.8) * 10.0);

            Particle {
                position: random_position,
                color: random_color,
                direction_vec: direction,
                particles: None,
                is_from_left: true,
                trail: LinkedList::new(),
                size: 3.0,
                exploded_at: None
            }
        }).collect();

        self.particles = match particles_vector.try_into() {
            Ok(particles) => Some(Box::new(particles)),
            Err(_) => panic!("Could not convert vector into array!")
        };
    }
}




