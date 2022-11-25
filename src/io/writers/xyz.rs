use std::fs::{self, File};
use std::io::prelude::*;
use std::io::LineWriter;

use crate::{
  io::writers::core::Writer,
  particle::Particle,
};

pub trait XYZ {
  fn write(&self);
}

impl XYZ for Writer {
  // TODO: check that this works
  fn write(&self) {
    let file = File::create(self.fname)?;
    let mut file = LineWriter::new(file);

    for particle in self.system.particles {
      let line = particle_to_line(&particle);
      file.write_all(line)?;
      file.write_all(b"\n")?;
    }
  }
}

fn particle_to_line(particle: &Particle) -> String {
  // TODO: implement
  Ok(())
}