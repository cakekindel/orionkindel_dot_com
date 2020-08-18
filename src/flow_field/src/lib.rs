use wasm_bindgen::prelude::*;
use js_sys::Math::random as js_random;

mod constants;
mod vector;
mod state;
mod particle;

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

    unsafe { STATE = Some(state); }

    log(&format!("flow_field: Initialized WASM rendering module").into());
    log(&format!(
        "flow_field: canvas size {}x{}",
        win_w,
        win_h
    ).into());
}

#[wasm_bindgen]
pub fn tick(draw_particle_cb: &js_sys::Function) -> () {
    let js_this = JsValue::NULL;
    let draw_particle = |p: &particle::Particle| {
        let p1 = js_sys::Object::new();
        js_sys::Reflect::set(&p1, &"x".into(), &p.pos.x.into());
        js_sys::Reflect::set(&p1, &"y".into(), &p.pos.y.into());

        let p2 = js_sys::Object::new();
        js_sys::Reflect::set(&p2, &"x".into(), &p.pos_prev.x.into());
        js_sys::Reflect::set(&p2, &"y".into(), &p.pos_prev.y.into());

        draw_particle_cb.call2(
            &js_this,
            &p1,
            &p2,
        ).unwrap();
    };

    unsafe { STATE.as_mut().unwrap().tick(&draw_particle); }
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

trait ConstrainWrap
    where Self : Sized
{
    fn constrain_wrap(self, max: impl Into<Self>) -> Self;
}
impl ConstrainWrap for f64 {
    fn constrain_wrap(self, max: impl Into<Self>) -> Self {
        let max = max.into();
        if self < 0.0 {
            max
        } else if self > max {
            0.0
        } else {
            self
        }
    }
}
