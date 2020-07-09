

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
pub type R = Result<()>;

/*
	This code was taken from Jacob's rust projects.
*/

pub trait ResultExt<T> {
  fn ann_err(self, msg: impl std::fmt::Display) -> std::result::Result<T, String>;
}

impl<T, E: std::fmt::Display> ResultExt<T> for std::result::Result<T, E> {
  fn ann_err(self, msg: impl std::fmt::Display) -> std::result::Result<T, String> {
    self.map_err(ann(msg))
  }
}

pub fn ann<T: std::fmt::Display>(s: impl std::fmt::Display) -> impl (Fn(T) -> String) {
  move |t| format!("{}: {}", s, t)
}
