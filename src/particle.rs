///
/// A container for a single particle instance describing its state
///
/// # Usage
///
/// ```
/// let particle = Particle::new();
/// ```
///

#[derive(Debug)]
pub struct Particle {
  pub mass: f32,
  pub pos: [f32; 3],
  pub vel: [f32; 3],
  pub force: [f32; 3],
}

impl Particle {
  pub fn new() -> Self {
    Particle {
      mass: 1.0,
      pos: [0.0, 0.0, 0.0],
      vel: [0.0, 0.0, 0.0],
      force: [0.0, 0.0, 0.0],
    }
  }
}

#[cfg(test)]
mod tests {

  use crate::particle::Particle;

  #[test]
  fn initialises() {
    let p = Particle::new();
    assert_eq!(p.mass, 1.0)
  }
}
