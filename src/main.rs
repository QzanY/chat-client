use tokio::{net::TcpStream,io::{BufReader,AsyncBufReadExt, AsyncWriteExt}};

#[tokio::main]
async fn main()
{
    println!("Enter you name : ");
    let stdin = tokio::io::stdin();
    let mut wrt_reader = BufReader::new(stdin);
    let mut name = String::new();
    wrt_reader.read_line(&mut name).await.unwrap();
    let mut name = name.split("\n").next().unwrap().to_string();
    let name_raw = name.clone();
    name.push_str(": ");
    let mut flag = true;

    if let Ok(mut stream) = TcpStream::connect("localhost:8080").await
    {
        let (reader,mut writer) = stream.split();

        let mut buf_read = BufReader::new(reader);
        let mut buffer = String::new();
        let mut wrt_buf = String::new();

        loop {
            if flag
            {
                writer.write(format!("{} joined the chat!\n",name_raw).as_bytes()).await.unwrap();
                writer.flush().await.unwrap();
                flag = false;
                println!("Welcome to the chat!");
            }
            wrt_buf.push_str(&name.to_string());
            tokio::select! {
                result = buf_read.read_line(&mut buffer) => {
                    if result.unwrap() == 0
                    {
                        break;
                    }
                    print!("{buffer}");
                    buffer.clear();
                }
                _result = wrt_reader.read_line(&mut wrt_buf) => {
                    writer.write_all(&wrt_buf.as_bytes()).await.unwrap();
                    wrt_buf.clear();
                }
            }
        }
    }
}