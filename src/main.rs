use tokio::net::{TcpListener, TcpStream};
use tokio_stream::StreamExt;
use modular_mc::util::*;
use modular_mc::packet::MyPacketDecoder;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:25565").await?;
    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            process(socket)
                .await
                .unwrap_or_else(|err| eprintln!("{:?}", err));
        });
    }
}

async fn process(socket: TcpStream) -> anyhow::Result<()> {
    let value = tokio_util::codec::Framed::new(socket, MyPacketDecoder).next().await.unwrap().unwrap();
    Ok(())
}
