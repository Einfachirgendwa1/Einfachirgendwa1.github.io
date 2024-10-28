use std::panic;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn initialize_new() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
pub fn readme_markdown() -> String {
    include_str!("../README.md").to_string()
}

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
        "f({x}) = {lagrange_value_for_x}\nf(x) = {lagrange_formula}\n\nwhere:\n{points}",
        lagrange_value_for_x = do_lagrange(x, &points).to_string(),
        lagrange_formula = lagrange_formula(&points),
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

fn lagrange_formula(points: &Vec<Point>) -> String {
    let mut output = String::new();
    for point in points {
        output.push_str(&format!("{} * ", point.y));

        output.push_str(
            points
                .iter()
                .filter(|inner_point| inner_point.x != point.x)
                .map(|inner_point| {
                    format!(
                        "((x - {inner_x}) / ({point_x} - {inner_x}))",
                        inner_x = inner_point.x,
                        point_x = point.x
                    )
                })
                .collect::<Vec<String>>()
                .join(" + ")
                .as_str(),
        );
        todo!()
    }

    output
}
