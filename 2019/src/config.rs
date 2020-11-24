use serde::de::DeserializeOwned;
use std::path::PathBuf;

pub struct Context<T> {
  fname: String,
  pub config: T,
}

impl<T> Context<T> {
  fn new(fname: String, config: T) -> Context<T> {
    Context { fname, config }
  }

  pub fn load_data(&self, fname: &str) -> Result<String, String> {
    let mut data_path = PathBuf::from(&self.fname);
    data_path.set_file_name(fname);

    let data = std::fs::read_to_string(data_path.as_os_str());

    match data {
      Ok(d) => Ok(d),
      Err(e) => Err(format!("Error reading file '{}': {:?}", fname, e)),
    }
  }
}

pub struct ContextFactory {
  fname: String,
}

impl ContextFactory {
  pub fn new(fname: String) -> ContextFactory {
    ContextFactory { fname }
  }

  pub fn create<T>(&self) -> Result<Context<T>, String>
  where
    T: DeserializeOwned,
  {
    let content = std::fs::read_to_string(&self.fname);
    if content.is_err() {
      return Err(format!("Could not load file at {}", self.fname));
    }

    let data: Result<T, _> = serde_yaml::from_str(&content.unwrap());
    if data.is_err() {
      return Err(format!("Error reading file {}", self.fname));
    }

    Ok(Context::new(self.fname.clone(), data.unwrap()))
  }
}
