use std::vec;
use crate::particle;

#[derive(Debug)]
pub struct System {
  pub dims: [f32; 3],
  pub particles: Vec<particle::Particle>
}

impl System {
  pub fn new(dims: [f32; 3]) -> Self {
    System{
      dims: dims,
      particles: vec![]
    }
  }
}