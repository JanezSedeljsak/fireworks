extern crate nannou;
mod particle;
mod trail;

use nannou::prelude::*;
use nannou::color::rgba;
use particle::*;

const PARTICLES_SIZE: usize = 6;

struct State {
    particles: [Particle; PARTICLES_SIZE],
}

fn main() {
    nannou::app(initialize)
        .update(update)
        .simple_window(view)
        .size(1280, 720)
        .run();
}

fn initialize(_app: &App) -> State {
    _app.main_window().set_title("Happy 2024!");
    let particles_vector: Vec<Particle> = (0..PARTICLES_SIZE).map(|_| Particle::new()).collect();
    let particles = match particles_vector.try_into() {
        Ok(particles) => particles,
        Err(_) => panic!("Could not convert vector into array!")
    };
    State { particles }
}

fn update(_app: &App, state: &mut State, _update: Update) {
    state.particles.iter_mut().for_each(|particle| particle.update());
}

fn view(_app: &App, state: &State, frame: Frame) {
    let back_color: Rgba = rgba(0.1, 0.1, 0.13, 1.0);
    let draw = _app.draw();
    draw.background().color(back_color);
    state.particles.iter().for_each(|particle| particle.draw(&draw));
    draw.to_frame(_app, &frame).unwrap();
}

