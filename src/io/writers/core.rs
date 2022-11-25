use system::System;

///
/// A base class for writing external files
///
/// # Usage
///
/// ```
/// let writer = Writer::new(system: System);
/// ```
///

#[derive(Debug)]
pub struct Writer {
  pub system: system::System,
  pub fname: String
}

impl Writer {
  pub fn new(system: &System, fname: &String) -> Self {
    Writer{
      system: system,
      fname: fname.to_string(),
    }
  }
}