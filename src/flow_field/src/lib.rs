use js_sys::Math::random as js_random;
use wasm_bindgen::prelude::*;

mod constants;
mod particle;
mod state;
mod vector;

use state::State;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &JsValue);
}

// TODO: should this be a mutex?
static mut STATE: Option<State> = None;

#[wasm_bindgen]
pub fn setup(win_w: i16, win_h: i16) -> () {
    console_error_panic_hook::set_once();

    let state = State::new(win_w, win_h);

    unsafe {
        if let Some(_) = STATE {
            panic!("flow_field::setup must only be called once.");
        }

        STATE = Some(state);
    }

    log(&format!("flow_field: Initialized WASM rendering module").into());
    log(&format!("flow_field: canvas size {}x{}", win_w, win_h).into());
}

#[wasm_bindgen]
pub fn tick(draw_particle_cb: &js_sys::Function) -> () {
    let js_this = JsValue::NULL;
    let draw_particle = |p: &particle::Particle| {
        draw_particle_cb
            .call2(&js_this, &p.pos.into(), &p.pos_prev.into())
            .unwrap();
    };

    unsafe {
        if let None = STATE {
            panic!("flow_field::setup must be called before flow_field::tick.");
        }

        STATE.as_mut().unwrap().tick(&draw_particle);
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Window {
    pub width: i16,
    pub height: i16,
}

pub fn rand_max(n: i16) -> i16 {
    let rand = js_random();

    let rand = rand * Into::<f64>::into(n);

    // this cast should be safe, since `js_random`
    // returns a value between 0 and 1, meaning
    // the multiplied `rand` will be at most `n`,
    // which is an i16
    rand.floor() as i16
}

trait WrappingClamp
where
    Self: Sized + Default + PartialOrd,
{
    /// # Wrapping Clamp
    /// Given a value (self), and a max,
    /// clamp the value between zero (Default) and max.
    ///
    /// If the value exceeds max, the value is wrapped to 0.
    ///
    /// If it is below Default::default(), the value is wrapped to max.
    fn wramp(self, max: Self) -> Self {
        if self < Default::default() {
            max
        } else if self > max {
            Default::default()
        } else {
            self
        }
    }
}

impl WrappingClamp for f64 {}
