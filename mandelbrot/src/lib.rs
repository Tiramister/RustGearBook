#![allow(clippy::unused_unit)]
use wasm_bindgen::prelude::wasm_bindgen;

/// c=x0+y0*i のとき、絶対値が2を超える反復回数
fn get_n_diverged(x0: f64, y0: f64, max_iter: usize) -> u8 {
    let mut xi = 0.0;
    let mut yi = 0.0;
    for i in 1..max_iter {
        let (x_next, y_next) = (xi * xi - yi * yi + x0, xi * yi * 2.0 + y0);
        xi = x_next;
        yi = y_next;

        if xi * xi + yi * yi > 4.0 {
            return i as u8;
        }
    }
    max_iter as u8
}

/// w*hのキャンバスの、各マスの色情報を返す
/// {x,y}_{min,max}はキャンバスの端の座標
/// max_iterは最大反復回数
#[wasm_bindgen]
pub fn generate_mandelbrot_set(
    canvas_w: usize,
    canvas_h: usize,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    max_iter: usize,
) -> Vec<u8> {
    let mut data = vec![];
    for i in 0..canvas_h {
        let y = y_min + (y_max - y_min) * (i as f64) / (canvas_w as f64);
        for j in 0..canvas_w {
            let x = x_min + (x_max - x_min) * (j as f64) / (canvas_h as f64);

            let iter_index = get_n_diverged(x, y, max_iter);
            let v = iter_index % 8 * 32;
            data.push(v); // R
            data.push(v); // G
            data.push(v); // B
            data.push(255); // A
        }
    }
    data
}
