use serde::{Deserialize, Serialize};
use super::{ConstrainWrap, vector::Vector, rand_max, Window, constants};
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Clone, Deserialize, Serialize)]
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

#[derive(Clone, Copy, Deserialize, Serialize)]
pub struct Particle {
    pub pos_prev: Vector,
    pub pos: Vector,
    pub vel: Vector,
    pub accel: Vector,
}
impl Particle {
    pub fn apply_force(&mut self, noise: &noise::Perlin, z_offset: f64) {
        use noise::NoiseFn;

        #[inline(always)]
        fn scale(n: f64) -> f64 {
            let div_by_scale = n / Into::<f64>::into(constants::SCALE);
            div_by_scale * constants::OFF_SCALE
        };

        let x = scale(self.pos.x);
        let y = scale(self.pos.y);

        let angle = noise.get([x, y, z_offset]);

        self.accel = Vector::from_angle(angle)
            .set_mag(constants::MAGNITUDE);
    }

    pub fn add_vel(&mut self) {
        self.vel = self.vel
            .add(self.accel)
            .max_mag(constants::MAX_SPEED)
    }

    pub fn update_pos(&mut self, window: Window) {
        let (win_w, win_h) = (window.width, window.height);

        let new_pos = self.pos.add(self.vel);

        let new_pos_clamp = Vector::new(
            new_pos.x.constrain_wrap(win_w),
            new_pos.y.constrain_wrap(win_h),
        );

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
