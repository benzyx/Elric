
use crate::utils::*;
use tokio::prelude::*;




const N_LENGTH_BYTES: usize = 4;
const LENGTH_LIMIT: usize = 100_000;

pub async fn framed_read<T: serde::de::DeserializeOwned>(stream: &mut (impl AsyncRead + Unpin)) -> Result<T> {
  let mut length_bytes = [0; N_LENGTH_BYTES];
  stream.read_exact(&mut length_bytes).await?;
  let length: u32 = bincode::deserialize(&length_bytes)?;
  let length = length as usize;
  if length > LENGTH_LIMIT {
    return Err(format!("reading: messages cannot be longer than {} bytes", LENGTH_LIMIT).into());
  }
  let mut buffer = vec![0; length];
  if length > 0 {
    stream.read_exact(&mut buffer).await.ann_err("reading message")?;
  }
  Ok(bincode::deserialize(&buffer).ann_err("deserializing message")?)
}


pub async fn framed_write<T: serde::Serialize>(stream: &mut (impl AsyncWrite + Unpin), msg: &T) -> R {
  let msg = bincode::serialize(msg).ann_err("serializing message")?;
  if msg.len() > LENGTH_LIMIT {
    return Err(format!("writing: messages cannot be longer than {} bytes", LENGTH_LIMIT).into());
  }
  let length: u32 = msg.len() as u32;
  stream
    .write_all(&bincode::serialize(&length).ann_err("serializing message")?)
    .await
    .ann_err("writing length")?;
  if length > 0 {
    stream.write_all(&msg).await.ann_err("writing message")?;
  }
  Ok(())
}