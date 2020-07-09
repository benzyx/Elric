mod event;
mod exchange;
mod orderbook;
mod types;
mod utils;
mod server;
mod protocol;
mod messages;

use std::sync::Arc;
use tokio::sync::Mutex;

use crate::exchange::*;
use crate::server::Server;
extern crate log;

#[tokio::main]
async fn main() {

  let symbols = "GOOG,MSFT";
  let symbols_vec: Vec<String> = symbols.split(',').map(|s| s.to_string()).collect();
  
  // Create the exchange.
  let exch = Exchange::new(Some(symbols_vec));
  let result = Server{ exchange: Arc::new(Mutex::new(exch)) }.start().await;
  match result {
    Ok(()) => (),
    Err(e) => eprintln!("error: {}", e),
  }
}
