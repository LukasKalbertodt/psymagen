// extern crate wasm_bindgen;


use wasm_bindgen::{
    Clamped, JsValue,
    prelude::*,
};
use web_sys::ImageData;

const SIZE: usize = 4096;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn incrementing() -> Result<ImageData, JsValue> {
    console_log!("start");

    let mut data = vec![255; SIZE * SIZE * 4];

    for i in (0..SIZE * SIZE).map(|i| i * 4) {
        data[i + 1] = 0;
        data[i + 0] = 0;
    }

    ImageData::new_with_u8_clamped_array(Clamped(&mut data), SIZE as u32)
}
