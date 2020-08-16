use serde::{Deserialize, Serialize};
use super::{Window, particle, constants};
use particle::{Particle, Particles};

#[derive(Deserialize, Serialize)]
pub struct State {
    pub noise_seed: u32,
    pub window: Window,
    pub z_offset: f64,
    pub show_grid: bool,
    pub show_colors: bool,
    pub particles: Particles,
}
impl State {
    pub fn new(win_w: i16, win_h: i16) -> Self {
        let window = Window { width: win_w, height: win_h };
        Self {
            noise_seed: noise::Perlin::DEFAULT_SEED,
            window,
            z_offset: 0.0,
            particles: Particles::new(window),
            show_colors: false,
            show_grid: false,
        }
    }

    pub fn tick(&mut self) -> () {
        use noise::Seedable;

        let window = self.window;

        let noise = noise::Perlin::new();
        noise.set_seed(self.noise_seed);

        let z_offset = &self.z_offset;
        self.particles
            .0
            .iter_mut()
            .for_each(|p: &mut Particle| {
                p.apply_force(&noise, *z_offset);
                p.add_vel();
                p.update_pos(window);
                p.reset_accel();
            });

        self.z_offset = self.z_offset + constants::TICK_CHANGE_AMOUNT;

        self.noise_seed = noise.seed();
    }
}
