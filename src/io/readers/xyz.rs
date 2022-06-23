mod core::CoreReader;

///
/// A set of routines for integrating a simulation system through time.
///
/// # Usage
///
/// ```
/// let reader = XYZReader::new("test.xyz");
/// ```
///

#[derive(Debug, CoreReader)]
pub struct XYZReader {
  fname: &str
}

impl XYZReader {
  pub fn new(fname: &str) -> Self {
    XYZReader{
      fname: fname,
    }
  }
}
