use crate::{particle::Particle, system::System};

///
/// A set of routines for integrating a simulation system through time.
///
/// # Usage
///
/// ```
/// let integrator = Integrator::new(0.1);
/// ```
///

#[derive(Debug)]
pub struct Integrator {
  timestep: f32,
}

impl Integrator {
  pub fn new(timestep: f32) -> Self {
    Integrator{
      timestep: timestep,
    }
  }

  pub fn run(&self, system: &mut System, steps: i32) {
    for _ in 0..steps {
      self.update(system)
    }
  }

  pub fn update(&self, system: &mut System) {
    self.velocity_verlet_initial(&mut system.particles);
    self.velocity_verlet_intermediate(&mut system.particles);
    self.velocity_verlet_final(&mut system.particles);
    system.pbc();

    // if output condition met, do output
  }

  fn velocity_verlet_initial(&self, particles: &mut Vec<Particle>) {
    for particle in particles.iter_mut() {
      for i in 0..3 {
        particle.vel[i] += self.timestep * 0.5 * particle.force[i] / particle.mass;
      }
    }
  }

  fn velocity_verlet_intermediate(&self, particles: &mut Vec<Particle>) {
    for particle in particles.iter_mut() {
      for i in 0..3 {
        particle.pos[i] += particle.vel[i] * self.timestep;
      }
    }
  }

  fn velocity_verlet_final(&self, particles: &mut Vec<Particle>) {
    for particle in particles.iter_mut() {
      for i in 0..3 {
        particle.vel[i] += self.timestep * 0.5 * particle.force[i] / particle.mass;
      }
    }
  }
}

#[cfg(test)]
mod tests {

  use crate::integrator::Integrator;

  #[test]
  fn initialises() {
    let i = Integrator::new(0.1);
    assert_eq!(i.timestep, 0.1)
  }
}