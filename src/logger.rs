
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum LogLevel {
  Trace,
  Debug,
  Info,
  Warn,
  Error,
}