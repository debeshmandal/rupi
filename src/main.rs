mod particle;
mod system;
mod integrator;

fn main() {
  println!("Welcome to rupi!")
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