use macroquad::prelude::*;

pub fn line_smoothing(points: &mut Vec<Vec2>) {
    let raw_points: Vec<Vec2> = points.clone();

    let filtered = remove_nearby_points(&raw_points, 2.0);
    let filtered2 = remove_colinear_points(&filtered, 0.05); // ~3 degrees
    let filtered3 = smooth_points(&filtered2, 0.2, 3);
    let final_points = remove_colinear_points(&filtered3, 0.05); // ~3 degrees

    *points = final_points;
}

pub fn remove_nearby_points(points: &Vec<Vec2>, min_distance: f32) -> Vec<Vec2> {
    if points.len() < 3 {
        return points.clone();
    }

    let mut cleaned = vec![points[0]];

    for i in 1..points.len() - 1 {
        let p: Vec2 = points[i];

        if cleaned
            .last()
            .map_or(true, |last: &Vec2| last.distance(p) >= min_distance)
        {
            cleaned.push(p);
        }
    }

    cleaned.push(*points.last().unwrap());

    cleaned
}

pub fn is_colinear(a: Vec2, b: Vec2, c: Vec2, tolerance: f32) -> bool {
    let ab = b - a;
    let bc = c - b;
    let angle = ab.angle_between(bc).abs();
    angle < tolerance
}

pub fn remove_colinear_points(points: &Vec<Vec2>, angle_tolerance: f32) -> Vec<Vec2> {
    if points.len() < 3 {
        return points.clone();
    }

    let mut cleaned = vec![points[0]];
    for i in 1..points.len() - 1 {
        let prev = cleaned.last().unwrap();
        let curr = points[i];
        let next = points[i + 1];

        if !is_colinear(*prev, curr, next, angle_tolerance) {
            cleaned.push(curr);
        }
    }
    cleaned.push(*points.last().unwrap());
    cleaned
}

pub fn smooth_points(points: &[Vec2], strength: f32, iterations: usize) -> Vec<Vec2> {
    let mut result = points.to_vec();

    for _ in 0..iterations {
        let mut new_points = result.clone();
        for i in 1..result.len() - 1 {
            let prev = result[i - 1];
            let curr = result[i];
            let next = result[i + 1];

            // Average neighbors and move current point slightly toward the average
            let target = (prev + next) * 0.5;
            new_points[i] = curr.lerp(target, strength);
        }
        result = new_points;
    }

    result
}
