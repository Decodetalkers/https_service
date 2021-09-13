use std::thread;
use tokio::{
    io,
    net::{TcpListener, TcpStream},
    select,
};
async fn proxy(client: &str, server: &str) -> io::Result<()> {
    let listener = TcpListener::bind(client).await?;
    loop {
        let (client, _) = listener.accept().await?;
        let server = TcpStream::connect(server).await?;
        thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async move {
                let (mut eread, mut ewrite) = client.into_split();
                let (mut oread, mut owrite) = server.into_split();

                let e2o = tokio::spawn(async move { io::copy(&mut eread, &mut owrite).await });
                let o2e = tokio::spawn(async move { io::copy(&mut oread, &mut ewrite).await });

                // let e2o = io::copy(&mut eread, &mut owrite);
                // let o2e = io::copy(&mut oread, &mut ewrite);

                select! {
                        _ = e2o => println!("c2s done"),
                        _ = o2e => println!("s2c done"),

                }
            });
        });
    }
}
#[tokio::main]
async fn main() -> io::Result<()> {
    proxy("127.0.0.1:9000", "127.0.0.1:8000").await
}
