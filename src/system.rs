use std::vec;
use crate::particle::Particle;

#[derive(Debug)]
pub struct System {
  pub dims: [f32; 3],
  pub particles: Vec<Particle>
}

impl System {
  pub fn new(dims: [f32; 3]) -> Self {
    System{
      dims: dims,
      particles: vec![]
    }
  }

  pub fn pbc(&mut self) {
    for particle in self.particles.iter_mut() {
			for i in 0..3 {
				if particle.pos[i] > self.dims[i] * 0.5 {
					particle.pos[i] -= self.dims[i];
				} else if -particle.pos[0] >= self.dims[0] * 0.5 {
					particle.pos[i] += self.dims[i];
				}
			}
		}
  }
}

#[cfg(test)]
mod tests {

  use crate::system::System;

  #[test]
  fn initialises() {
    let s = System::new([10.0, 10.0, 10.0]);
    assert_eq!(s.dims[0], 10.0)
  }
}