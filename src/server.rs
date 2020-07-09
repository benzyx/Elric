use crate::utils::*;
use crate::messages::*;
use crate::messages::ClientMessage::*;
use crate::protocol::*;
use crate::exchange::Exchange;

use log::*;
use std::sync::Arc;

#[allow(unused_imports)]
use tokio::prelude::*;
use tokio::stream::StreamExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;



const LOCALHOST: &str = "127.0.0.1";

#[derive(Clone)]
pub struct Server {
  pub exchange: Arc<Mutex<Exchange>>,
  // pub user_positions: Arc<UserPositions>,
}

impl Server {
  pub async fn start(self) -> R {
    let mut listener = TcpListener::bind((LOCALHOST, 3125)).await?;
    let _port = listener.local_addr()?.port();
    tokio::spawn(async move {
      let mut incoming = listener.incoming();
      while let Some(Ok(stream)) = incoming.next().await {
        
        tokio::spawn({
          let self_clone = self.clone();
          async move {
            self_clone.handle_connection(stream).await.map_err(|e| {
            info!("Server connection lost: {}", e);
          })}
        });
      }
    });
    Ok(())
  }

  async fn handle_connection(self, mut stream: TcpStream) -> R {
    let msg: ClientMessage = framed_read(&mut stream).await?;
    match msg {
      LimitOrderMsg(limit_order_msg) => {
        self.exchange.lock().await.process_limit_order(limit_order_msg);
      }
    };
    Ok(())
  }

}





