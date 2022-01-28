use gloo_console::log;
use js_sys::Function;
use wasm_bindgen::{prelude::*, JsCast};

#[wasm_bindgen]
extern "C" {
    type Global;

    #[wasm_bindgen(method, js_name = "clearTimeout")]
    fn clear_timeout(this: &Global, handle: i32);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "clearTimeout")]
    fn clear_timeout(handle: i32);

    #[wasm_bindgen(js_name = "setTimeout", catch)]
    fn set_timeout(handler: &Function, timeout: i32) -> Result<i32, JsValue>;
}

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    let c = Closure::once(|| log!("should not show up"));
    let f = c.as_ref().unchecked_ref::<js_sys::Function>();
    let handle = set_timeout(f, 1_000).expect("no handle");
    // This works even under webpack
    js_sys::global().unchecked_ref::<Global>().clear_timeout(handle);
    log!("No death yet (the first timer was cleared)");

    let c = Closure::once(|| log!("This timer was not cleared"));
    let f = c.as_ref().unchecked_ref::<js_sys::Function>();
    let handle = set_timeout(f, 1_000).expect("no handle");
    clear_timeout(handle);
    log!("will not get here");
    Ok(())
}
