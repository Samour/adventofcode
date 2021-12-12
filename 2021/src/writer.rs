pub enum Writer {
  StdoutWriter,
  NoopWriter,
}

impl Writer {
  pub fn write<F>(&self, generate: F)
  where
    F: FnOnce() -> String,
  {
    match self {
      Self::StdoutWriter => println!("{}", generate()),
      Self::NoopWriter => {}
    }
  }
}
