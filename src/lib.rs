use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn lagrange(input_for_x: &str, input_for_points: &str) -> String {
    let mut points = Vec::new();

    if input_for_x.is_empty() || input_for_points.is_empty() {
        return "".to_string();
    }

    let x = match input_for_x.parse() {
        Ok(value) => value,
        Err(err) => return format!("Failed to parse the value for x: `{input_for_x}`: `{err}`"),
    };

    for (this, e) in input_for_points
        .split(';')
        .filter(|this| !this.is_empty())
        .map(|this| (this, this.split_once(',')))
    {
        match e {
            Some((x, y)) => {
                let x = match x.parse() {
                    Ok(value) => value,
                    Err(err) => return format!("Failed to parse this number: `{x}`: `{err}`"),
                };
                let y = match y.parse() {
                    Ok(value) => value,
                    Err(err) => return format!("Failed to parse this number: `{y}`: `{err}`"),
                };
                points.push(Point { x, y });
            }
            None => return format!("Failed to parse this point: {this} (did you forget a comma?)"),
        }
    }

    format!(
        "f({x}) = {lagrange}\n\nwhere:\n{points}",
        lagrange = do_lagrange(x, &points).to_string(),
        points = points
            .iter()
            .map(|point| format!("\tf({x}) = {y}\n", x = point.x, y = point.y))
            .collect::<String>()
    )
}

pub struct Point {
    x: f64,
    y: f64,
}

fn do_lagrange(x: f64, points: &Vec<Point>) -> f64 {
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
