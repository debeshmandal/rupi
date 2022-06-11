mod particle;
mod system;
mod integrator;

use std::time::Instant;

fn main() {
  println!("Welcome to rupi!");
  let now = Instant::now();

  let mut s = system::System::new([10.0, 10.0, 10.0]);
  let inte = integrator::Integrator::new(0.1);

  for _ in 0..32000 {
    let p = particle::Particle::new();
    s.particles.push(p);
  }
  inte.run(&mut s, 100);
  let elapsed = now.elapsed();
  println!("Simulation took {:?}ms", elapsed.as_micros() as f32 / 1000.0);
}

#[cfg(test)]
mod tests {

  use crate::{
    particle::Particle,
    system::System,
    integrator::Integrator,
  };

  #[test]
  fn simple_simulation() {
    let mut p = Particle::new();
    let mut s = System::new([10.0, 10.0, 10.0]);
    let i = Integrator::new(0.1);

    p.pos[0] += 1.0;
    s.particles.push(p);
    i.run(&mut s, 91);

    assert_eq!(s.particles[0].pos[1], 0.0)
  }
}