use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn lagrange(input_for_x: &str, input_for_points: &str) -> String {
    let mut points = Vec::new();

    let Ok(x) = input_for_x.parse() else {
        return format!("Failed to parse the value for x: `{input_for_x}`");
    };

    for (this, e) in input_for_points
        .split(';')
        .map(|this| (this, this.split_once(',')))
    {
        match e {
            Some((x, y)) => {
                let Ok(x) = x.parse() else {
                    return format!("Failed to parse this number: `{x}`.");
                };
                let Ok(y) = y.parse() else {
                    return format!("Failed to parse this number: `{y}`.");
                };
                points.push(Point { x, y });
            }
            None => return format!("Failed to parse this point: {this} (did you forget a comma?)"),
        }
    }

    do_lagrange(x, points).to_string()
}

pub struct Point {
    x: f64,
    y: f64,
}

fn do_lagrange(x: f64, points: Vec<Point>) -> f64 {
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
