use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Point {
    x: f64,
    y: f64,
}

#[wasm_bindgen]
pub fn lagrange(x: f64, points: Vec<Point>) -> f64 {
    points
        .iter()
        .map(|point| {
            point.y
                * points
                    .iter()
                    .filter(|inner_point| inner_point.x != point.x)
                    .map(|inner_point| (x - inner_point.x) / (point.x - inner_point.x))
                    .product::<f64>()
        })
        .sum()
}
