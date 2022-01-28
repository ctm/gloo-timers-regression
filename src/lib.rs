use {gloo_timers::callback::Timeout, wasm_bindgen::prelude::*};

const DELAY_MILLIS: u32 = 30_000; // 30 seconds

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    let mut _ping_timeout = Timeout::new(DELAY_MILLIS, move || {});
    _ping_timeout = Timeout::new(DELAY_MILLIS, move || {});
    Ok(())
}
