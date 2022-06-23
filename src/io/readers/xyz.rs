use crate::io::readers::core::Reader;

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
  fn load_xyz(&self);
}

impl XYZ for Reader {
  fn load_xyz(&self) {
    println!("{:?}", self.fname);
  }
}
