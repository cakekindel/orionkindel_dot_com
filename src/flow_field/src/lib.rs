use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use js_sys::Math::random as js_random;

mod constants;
mod vector;
mod state;
mod particle;

use state::State;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn setup(win_w: i16, win_h: i16) -> JsValue {
    console_error_panic_hook::set_once();

    let state = State::new(win_w, win_h);
    JsValue::from_serde(&state)
        .expect("failed to serialize State")
}

#[wasm_bindgen]
pub fn tick(state_js: JsValue) -> JsValue {
    let mut state: State = state_js.into_serde()
        .expect("failed to deserialize State");

    state.tick();
    log("ticked!");

    JsValue::from_serde(&state)
        .expect("failed to serialize State")
}

#[derive(Clone, Copy, Deserialize, Serialize)]
pub struct Window {
    pub width: i16,
    pub height: i16,
}

pub fn rand_max(n: i16) -> i16 {
    let rand = js_random();

    let rand = rand * Into::<f64>::into(n);
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
