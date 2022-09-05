
use anyhow::Result;
use async_prost::AsyncProstStream;
use futures::prelude::*;
use kv::{CommandRequest, CommandResponse};
use tokio::net::TcpListener;
use tracing::info;


#[warn(unused_imports)]
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let addr = "127.0.0.1:9527";
    let listener = TcpListener::bind(addr).await?;
    info!("Start listening on {}", addr);
    
    loop {
        let (stream, addr) = listener.accept().await?;
        info!("client {:?} connected", addr);

        tokio::spawn(async move {
            let mut steam =
                AsyncProstStream::<_, CommandRequest, CommandResponse, _>::
                from(stream).for_async();
            while let Some(Ok(msg)) = steam.next().await {
                info!("Got a new command: {:?}", msg);
                let mut resp = CommandResponse::default();
                resp.status = 404;
                resp.message = "Not found".to_string();
                steam.send(resp).await.unwrap();
            }
            info!("Client {:?} disconnected", addr); 
        });
    }

}