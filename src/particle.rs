#[derive(Debug)]
pub struct Particle {
  pub mass: f32,
  /*pub pos: [f32, 3],
  pub vel: [f32, 3],
  pub force: [f32, 3], */
}

impl Particle {
  pub fn new() -> Self {
    Particle{
      mass: 1.0
    }
  }
}
