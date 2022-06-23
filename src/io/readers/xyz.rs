use crate::io::readers::core::Reader;

///
/// A trait for reading XYZ files.
///
/// # Usage
///
/// ```
/// let reader = XYZReader::new("test.xyz");
/// ```
///

pub trait XYZ {
  fn load_xyz(&self);
}

impl XYZ for Reader {
  fn load_xyz(&self) {
    println!("{:?}", self.fname);
  }
}
