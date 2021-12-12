pub enum Writer {
  StdoutWriter,
  NoopWriter,
}

impl Writer {
  pub fn write<F, S>(&self, generate: F)
  where
    F: FnOnce() -> S,
    S: std::fmt::Display,
  {
    match self {
      Self::StdoutWriter => println!("{}", generate()),
      Self::NoopWriter => {}
    }
  }
}
