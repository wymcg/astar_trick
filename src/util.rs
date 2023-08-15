/// Get the distance between two points
pub fn euclidian_distance(a: (usize, usize), b: (usize, usize)) -> f32 {
    (((a.1 - b.1).pow(2) + (a.0 - b.0).pow(2)) as f32).sqrt()
}