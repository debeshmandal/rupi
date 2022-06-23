mod particle;
mod system;
mod integrator;
mod utils;
mod io;

use std::{time::Instant, env};


fn main() {
  println!("Welcome to rupi!");

  let args: Vec<String> = env::args().collect();
  let number = args[1].parse::<i32>().unwrap();

  let now = Instant::now();

  let mut s = system::System::new([10.0, 10.0, 10.0]);
  let inte = integrator::Integrator::new(0.1);

  // if particles file given, read particles
  let reader = io::readers::core::Reader::new("test.xyz".to_string());
  let xyz: &dyn io::readers::xyz::XYZ = &reader;
  xyz.load_xyz();

  for _ in 0..number {
    let p = particle::Particle::new();
    s.particles.push(p);
  }

  // set output period - to be later set by input script

  inte.run(&mut s, 100);
  let elapsed = now.elapsed();
  println!("Simulation took {:?}ms for {:?} particles", elapsed.as_micros() as f32 / 1000.0, number);
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