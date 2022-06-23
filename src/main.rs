mod particle;
mod system;
mod integrator;
mod utils;
mod io;

use std::{
  time::Instant,
  env
};


fn main() {
  println!("Welcome to rupi!");

  let args: Vec<String> = env::args().collect();
  let fname = &args[1];
  let timestep = 0.1;
  let steps = 1000;

  // handle arguments
  let now = Instant::now();

  let mut s = system::System::new([10.0, 10.0, 10.0]);
  let inte = integrator::Integrator::new(timestep);

  // if particles file given, read particles
  let reader = io::readers::core::Reader::new(&fname.to_string());
  let xyz: &dyn io::readers::xyz::XYZ = &reader;
  xyz.load_xyz(&mut s);

  // set output period - to be later set by input script
  println!("Running simulation with for {:?} steps with a timestep of {:?}", steps, timestep);
  inte.run(&mut s, steps);
  let elapsed = now.elapsed();
  println!("Simulation took {:?}ms for {:?} particles", elapsed.as_micros() as f32 / 1000.0, s.particles.len());
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