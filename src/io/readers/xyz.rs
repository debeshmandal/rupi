use crate::{
  io::readers::core::Reader,
  system::System,
  particle::Particle,
};

///
/// A trait for reading XYZ files.
///
/// # Usage
///
/// ```
/// let reader = Reader::new("test.xyz".to_string());
/// let xyz &dyn io::readers::xyz::XYZ = &reader;
/// xyz.load_xyz();
/// ```
///

pub trait XYZ {
  fn load_xyz(&self, s: &mut System);
}

impl XYZ for Reader {
  fn load_xyz(&self, s: &mut System) {

    // iterate over lines and pass each
    // line to line_to_particle
    let mut i = 0;
    for line in self.contents.lines() {
      if i > 1 {
        s.particles.push(line_to_particle(&line));
      }
      i += 1;
    }
  }
}

fn line_to_particle(line: &str) -> Particle {
  let v: Vec<&str> = line.split(" ").collect();
  let mut p = Particle::new();
  p.mass = v[0].parse::<f32>().unwrap();
  p.pos = [
    v[1].parse::<f32>().unwrap(),
    v[2].parse::<f32>().unwrap(),
    v[3].parse::<f32>().unwrap(),
  ];
  p
}