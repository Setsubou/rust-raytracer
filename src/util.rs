use rand::{thread_rng, Rng};

pub fn _degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}

pub fn random_double() -> f64 {
    let mut rng = thread_rng();

    rng.gen_range(0.0..1.0)
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
    let mut rng = thread_rng();

    rng.gen_range(min..max)
}
