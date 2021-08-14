use rand::Rng;

pub fn random_f64(min_max: Option<(f64, f64)>) -> f64 {
    let (min, max) = min_max.unwrap_or((0.0, 1.0));
    let mut rng = rand::thread_rng();
    rng.gen_range(min..=max)
}
