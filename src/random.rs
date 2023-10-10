use rand::Rng;

pub fn random_float(min: f32, max: f32) -> f32 {
    let mut rng = rand::thread_rng();
    min + rng.gen::<f32>() * (max - min)
}