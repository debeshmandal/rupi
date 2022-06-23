///
/// A base class for managing external files
///
/// # Usage
///
/// ```
/// let reader = CoreReader::new("test.xyz");
/// ```
///

#[derive(Debug)]
pub struct CoreReader {
  fname: &str
}

impl CoreReader {
  pub fn new(fname: &str) -> Self {
    CoreReader{
      fname: fname,
    }
  }
}
