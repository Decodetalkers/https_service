use std::thread;
use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    net::TcpStream,
    select,
};
use url::Url;
#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8000").await?;
    loop {
        let (client, addr) = listener.accept().await?;
        println!("address : {}", addr);
        //let (eread, ewrite)= client.into_split();
        let (mut eread, mut ewrite) = client.into_split();
        let mut recept = [0; 16384];

        //println!("hello");
        eread.read(&mut recept).await?;
        let input = String::from_utf8_lossy(&recept[..]);
        let mut vec = input.lines();
        let header = vec.next().unwrap();
        println!("{}", header);
        let mut header_split = header.split(' ');
        let func = header_split.next().unwrap().to_string();
        let url_may = header_split.next();
        if url_may == None {
            continue;
        }
        let url = url_may
            .unwrap()
            .to_string();
        //let mut adress: &str = "";
        let url1 = Url::parse(&url).unwrap();
        let adress = {
            if func == "CONNECT" {
                url1.scheme().to_string() + ":" + url1.path()
            } else {
                let mut addres = url1.scheme().to_string();
                //let mut add = String::new();
                if url1.port().is_none() {
                    addres = url1.host().unwrap().to_string() + ":80";
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
        thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async move {
                match tokio::time::timeout(
                    std::time::Duration::from_millis(40),
                    TcpStream::connect(&adress),
                )
                .await
                {
                    Ok(message) => {
                        match message {
                            Ok(server) => {
                                let (mut oread, mut owrite) = server.into_split();
                                //let mut turn = [0; 1000000];
                                if func == "CONNECT" {
                                    // https 的话要返回串
                                    ewrite
                                        .write(
                                            "HTTP/1.1 200 Connection established\r\n\r\n"
                                                .as_bytes(),
                                        )
                                        .await?;
                                } else {
                                    owrite.write(&recept).await?;
                                }

                                let s2c = tokio::spawn(async move {
                                    io::copy(&mut oread, &mut ewrite).await?;
                                    //server.read(&mut turn).await?;
                                    //println!("Retrun: {}", String::from_utf8_lossy(&turn[..]));
                                    //ewrite.write(&turn).await?;
                                    ewrite.flush().await
                                });
                                let c2s = tokio::spawn(async move {
                                    io::copy(&mut eread, &mut owrite).await?;
                                    owrite.flush().await
                                });
                                select! {
                                    _  = c2s => println!("c2s done"),
                                    _  = s2c => println!("s2c done"),

                                }
                            }
                            Err(_) => panic!("error"),
                        };
                    }
                    Err(_) => println!("timeout,{}", adress),
                }

                Ok::<(), io::Error>(())
            })
            .expect("ssss");
        });
    }
}
