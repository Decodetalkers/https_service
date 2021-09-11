use tokio::{
    io::{
        self,
        AsyncReadExt,
        AsyncWriteExt
    },
    net::TcpListener,
    net::TcpStream,
    select,
};
static UNKOWNURL: &str = include_str!("hello.html");
#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000").await?;
    loop{
        let (client,_)= listener.accept().await?;
        //let (eread, ewrite)= client.into_split();
        let (mut eread, mut ewrite) = client.into_split();
        let mut recept = [0; 1024];
        let o2e = tokio::spawn(async move {
            //println!("hello");
            eread.read(&mut recept).await?;
            println!("Request: {}", String::from_utf8_lossy(&recept[..]));
            //let contents = include_str!("hello.html");
            //let response = format!(
            //    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            //    contents.len(),
            //    contents
            //);
            let mut server = TcpStream::connect("www.baidu.com:80").await?;
            let mut turn = [0; 1000000];
            server.write(&recept).await?;
            server.read(&mut turn).await?;
            println!("Retrun: {}", String::from_utf8_lossy(&turn[..]));
            ewrite.write(&turn).await?;
            ewrite.flush().await
        });
        select! {
            _ = o2e => println!("finish"),
        }
    }
}
