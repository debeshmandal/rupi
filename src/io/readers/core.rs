use std::fs;

///
/// A base class for managing external files
///
/// # Usage
///
/// ```
/// let reader = Reader::new("test.xyz");
/// ```
///

#[derive(Debug)]
pub struct Reader {
  pub fname: String,
  pub contents: String
}

impl Reader {
  pub fn new(fname: &String) -> Self {
    let contents = fs::read_to_string(fname)
        .expect("Something went wrong reading the file");
    Reader{
      fname: fname.to_string(),
      contents: contents,
    }
  }
}
