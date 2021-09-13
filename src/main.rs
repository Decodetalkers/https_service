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
use url::Url;
#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000").await?;
    loop{
        let (client,addr)= listener.accept().await?;
        println!("address : {}",addr);
        //let (eread, ewrite)= client.into_split();

        tokio::spawn(async move  {
            let (mut eread, mut ewrite) = client.into_split();
            let mut recept = [0; 1024];

                //println!("hello");
            eread.read(&mut recept).await?;
            //println!("Request: {}", String::from_utf8_lossy(&recept[..]));
            let input = String::from_utf8_lossy(&recept[..]);
            let mut vec = input.lines();
            let header = vec.next().unwrap();
            let mut header_split = header.split(' ');
            let func = header_split.next().unwrap();
            let url = header_split.next().unwrap();
            //let mut adress: &str = "";
            let url1 = Url::parse(url).unwrap();
            let adress  = {
                if func == "CONNECT" {
                    url1.scheme().to_string() +":" + url1.path()
                }else {
                    let mut addres = url1.scheme().to_string();
                    //let mut add = String::new();
                    if url1.port().is_none(){
                        addres = url1.host().unwrap().to_string()+":80";
                    };
                    addres
                }
            };
            //let contents = include_str!("hello.html");
            //let response = format!(
            //    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            //    contents.len(),
            //    contents
            //);
            //超时直接返回error
            match tokio::time::timeout(
                std::time::Duration::from_secs(10),
                TcpStream::connect(&adress)
            ).await{
                Ok(message) => {
                    match message
                    {
                        Ok(server)=>{
                            let (mut oread, mut owrite) = server.into_split();
                            //let mut turn = [0; 1000000];
                            if func == "CONNECT"{
                                // https 的话要返回串
                                ewrite.write("HTTP/1.1 200 Connection established\r\n\r\n".as_bytes()).await?;
                            }else {
                                owrite.write(&recept).await?;
                            }

                            let s2c = tokio::spawn(async move {
                                io::copy(&mut oread, &mut ewrite).await?;
                                //server.read(&mut turn).await?;
                                //println!("Retrun: {}", String::from_utf8_lossy(&turn[..]));
                                //ewrite.write(&turn).await?;
                                ewrite.flush().await
                            });
                            let c2s = tokio::spawn(async move{
                                io::copy(&mut eread,&mut owrite).await?;
                                owrite.flush().await
                            });
                            select! {
                                _  = c2s => println!("c2s done"),
                                _  = s2c => println!("s2c done"),

                            }
                        },
                        Err(_)=> panic!("error"),
                    };
                },
                Err(_) => println!("timeout,{}",adress),
            }
            Ok::<(),io::Error>(())
        });
    }
}
