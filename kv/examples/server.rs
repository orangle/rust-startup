
use anyhow::Result;
use async_prost::AsyncProstStream;
use futures::prelude::*;
use kv::{CommandRequest, CommandResponse, Service, MemTable};
use tokio::net::TcpListener;
use tracing::info;


#[warn(unused_imports)]
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let addr = "127.0.0.1:9527";
    let service = Service::new(MemTable::new());
    let listener = TcpListener::bind(addr).await?;
    info!("Start listening on {}", addr);
    
    loop {
        let (stream, addr) = listener.accept().await?;
        info!("client {:?} connected", addr);
        let svc = service.clone();

        tokio::spawn(async move {
            let mut steam =
                AsyncProstStream::<_, CommandRequest, CommandResponse, _>::
                from(stream).for_async();

            while let Some(Ok(cmd)) = steam.next().await {
                info!("Got a new command: {:?}", cmd);
                let res = svc.execute(cmd);
                steam.send(res).await.unwrap();
            }
            info!("Client {:?} disconnected", addr); 
        });
    }

}