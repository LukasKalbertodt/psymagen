// extern crate wasm_bindgen;


use bit_set::BitSet;
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

macro_rules! println {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[derive(Debug)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn from_u32(color: u32) -> Self {
        assert!(color < 256u32.pow(3));

        let r = ((color & 0x00FF0000) >> 2 * 8) as u8;
        let g = ((color & 0x0000FF00) >> 1 * 8) as u8;
        let b = ((color & 0x000000FF) >> 0 * 8) as u8;

        Self { r, g, b }
    }
}

#[derive(Debug)]
struct Data(pub Vec<u8>);

impl Data {
    pub fn new() -> Self {
        let mut out = vec![0; SIZE * SIZE * 4];
        for i in (0..SIZE * SIZE).map(|i| i * 4) {
            out[i + 3] = 255;
        }

        Data(out)
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> &[u8] {
        let start = (y * SIZE + x) * 4;
        &self.0[start..(start + 4)]
    }

    pub fn get_pixel_mut(&mut self, x: usize, y: usize) -> &mut [u8] {
        let start = (y * SIZE + x) * 4;
        &mut self.0[start..(start + 4)]
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        let pixel = self.get_pixel_mut(x, y);

        pixel[0] = color.r;
        pixel[1] = color.g;
        pixel[2] = color.b;
        pixel[3] = 255;
    }

    pub fn contains_all_colors(&self) -> bool {
        let mut set = BitSet::new();

        for x in 0..SIZE {
            for y in 0..SIZE {
                let p = self.get_pixel(x, y);
                let id = (p[0] as u32) * 256 * 256 + (p[1] as u32) * 256 + (p[2] as u32);
                set.insert(id as usize);
            }
        }

        (0..SIZE*SIZE).all(|id| set.contains(id))
    }

    pub fn into_image_data(mut self) -> Result<ImageData, JsValue> {
        ImageData::new_with_u8_clamped_array(Clamped(&mut self.0), SIZE as u32)
    }
}


#[wasm_bindgen]
pub fn incrementing() -> Result<ImageData, JsValue> {
    println!("start");

    let mut color = 0u32;
    let mut data = Data::new();
    for x in 0..SIZE {
        for y in 0..SIZE {
            data.set_pixel(x, y, Color::from_u32(color));
            color += 1;
        }
    }

    println!("peter: {}", data.contains_all_colors());

    data.into_image_data()
}
