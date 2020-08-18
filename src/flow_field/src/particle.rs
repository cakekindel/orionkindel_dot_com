use wasm_bindgen::prelude::*;

use super::{constants, rand_max, vector::Vector, Window, WrappingClamp};

#[derive(Clone)]
pub struct Particles(pub Vec<Particle>);
impl Particles {
    pub fn new(window: Window) -> Self {
        use std::iter;

        let particles = iter::repeat(())
            .take(constants::PARTICLE_COUNT)
            .map(|_| (rand_max(window.width), rand_max(window.height)))
            .map(|(x, y)| Vector::new(x.into(), y.into()))
            .map(|vector| Particle {
                pos_prev: vector,
                pos: vector.clone(),
                vel: Vector::new(0.0, 0.0),
                accel: Vector::new(0.0, 0.0),
            })
            .collect();

        Self(particles)
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Particle {
    pub pos_prev: Vector,
    pub pos: Vector,
    pub vel: Vector,
    pub accel: Vector,
}
impl Particle {
    pub fn apply_force(&mut self, noise: &noise::Perlin, z_offset: f64) {
        use noise::NoiseFn;
        use nalgebra::RealField;

        #[inline(always)]
        fn scale(n: f64) -> f64 {
            let div_by_scale = n / f64::from(constants::SCALE);
            div_by_scale * constants::OFF_SCALE
        };

        let x = scale(self.pos.x);
        let y = scale(self.pos.y);

        let angle = noise.get([x, y, z_offset]) * f64::two_pi() * 4.0;

        self.accel = Vector::from_angle(angle).set_mag(constants::MAGNITUDE);
    }

    pub fn add_vel(&mut self) {
        self.vel = self.vel.add(self.accel).max_mag(constants::MAX_SPEED)
    }

    pub fn update_pos(&mut self, window: Window) {
        let (win_w, win_h) = (window.width, window.height);

        let new_pos = self.pos.add(self.vel);

        let new_pos_clamp =
            Vector::new(new_pos.x.wramp(win_w.into()), new_pos.y.wramp(win_h.into()));

        // If previous position was prior to a `wramp`,
        // set the previous position to the new position.
        // This is to prevent a particle that was wrapped
        // to another edge from being rendered as a horizontal
        // line spanning the canvas.
        self.pos_prev = if new_pos != new_pos_clamp {
            new_pos_clamp
        } else {
            self.pos
        };

        self.pos = new_pos_clamp;
    }

    pub fn reset_accel(&mut self) {
        self.accel = Vector::new(0.0, 0.0);
    }
}
