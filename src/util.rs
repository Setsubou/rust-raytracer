use rand::{thread_rng, Rng};

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}

pub fn random_double() -> f64 {
    thread_rng().gen_range(0.0..1.0)
}

pub fn random_double_with_range(min: f64, max: f64) -> f64 {
    thread_rng().gen_range(min..max)
}
