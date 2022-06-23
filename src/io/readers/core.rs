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
pub struct Reader {
  pub fname: String
}

impl Reader {
  pub fn new(fname: String) -> Self {
    Reader{
      fname: fname,
    }
  }
}
