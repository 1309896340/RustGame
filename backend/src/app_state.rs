#[derive(Clone)]
pub struct AppState {
  pub sqlite_file_path: String,
}

impl AppState {
  pub fn new(sqlite_file_path: &str) -> Self {
    AppState {
      sqlite_file_path: sqlite_file_path.to_string(),
    }
  }
}

