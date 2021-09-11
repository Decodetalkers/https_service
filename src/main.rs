use tokio::{
    io::{
        self,
        //AsyncReadExt,
        AsyncWriteExt
    },
    net::TcpListener,
    select,
};

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000").await?;
    loop{
        let (client,_)= listener.accept().await?;
        //let (eread, ewrite)= client.into_split();
        let (mut eread, mut ewrite) = client.into_split();
        let mut recept: Vec<u8> = vec![];
        let o2e = tokio::spawn(async move {
            println!("hello");
            //eread.read(&mut recept).await?;
            io::copy(&mut eread, &mut recept).await?;
            println!("{}",ascii_to_string(recept));
            ewrite.write(b"sss").await
            //ewrite.write(String::from_utf8_lossy(&buffer[..]).as_bytes()).await
            //io::copy(&mut eread, &mut recept).await
            //println!("{}",ascii_to_string(recept));
        });
        select! {
            _ = o2e => println!("finish"),
        }
    }
}
#[allow(dead_code)]
fn ascii_to_char(code: u8) -> char {
    std::char::from_u32(code as u32).unwrap_or('_')
}
#[allow(dead_code)]
fn ascii_to_string(code: Vec<u8>) -> String {
    let mut url = String::new();
    for count in code {
        url.push(ascii_to_char(count));
    }
    url
}

