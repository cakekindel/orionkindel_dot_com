use super::{constants, particle, Window};
use particle::{Particle, Particles};

pub struct State {
    pub noise: noise::Perlin,
    pub window: Window,
    pub z_offset: f64,
    pub show_grid: bool,
    pub show_colors: bool,
    pub particles: Particles,
}
impl State {
    pub fn new(win_w: i16, win_h: i16) -> Self {
        let window = Window {
            width: win_w,
            height: win_h,
        };
        Self {
            noise: noise::Perlin::new(),
            window,
            z_offset: 0.0,
            particles: Particles::new(window),
            show_colors: false,
            show_grid: false,
        }
    }

    pub fn tick(&mut self, draw_particle: &impl Fn(&Particle) -> ()) -> () {
        let window = self.window;
        let noise = &self.noise;
        let z_offset = &self.z_offset;

        self.particles
            .0
            .iter_mut()
            // HOT LOOP
            .for_each(|p: &mut Particle| {
                p.apply_force(noise, *z_offset);
                p.add_vel();
                p.update_pos(window);
                p.reset_accel();

                draw_particle(p);
            });

        self.z_offset += constants::TICK_CHANGE_AMOUNT;
    }
}
