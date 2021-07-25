use std::net::{TcpListener, TcpStream, Shutdown};
use std::thread;
use std::time;
use std::io::{self, Error,prelude::*};
// use std::str;


fn main() {
    let handle_server = thread::spawn(move || {
        println!("[server]: start");
        worker_server().unwrap();
        println!("[server]: close");
    });

    let handle_client = thread::spawn(move || {
        println!("[Client]: start");
        worker_client().unwrap();
        println!("[Client]: close");
    });


    // 先启用 服务端 任务，未完成前 阻塞
    handle_server.join().unwrap();
    handle_client.join().unwrap();



}

fn worker_server() -> std::io::Result<()> {
    // ?为match快捷 写法
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    for stream in listener.incoming() {
        let stream = stream.expect("failed!");
        //处理stream
        handle_client(stream)
		    .unwrap_or_else(|error| eprintln!("{:?}", error));
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream) -> Result<(), Error>{
    
    let mut buf = [0; 512];
    // 把内容读到 缓存 buf里
    // bytes_read 存长度
    let bytes_read = stream.read(&mut buf)?;
    if bytes_read == 0 {
        return Ok(());
    }
    // 单纯的打印
    println!("[server]: get Message: {}", String::from_utf8_lossy(&buf[..bytes_read]));
    // 返回给 客户端
    stream.write(b"Server received message")?;
    thread::sleep(time::Duration::from_secs(1 as u64));


    Ok(())
}


fn worker_client() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;

    let mut input = String::new();
    // 控制台输入 消息
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read from stdin");

    let _status = stream.write(input.as_bytes())
        .expect("Failed to write to stream");

    let mut buf = [0; 512];
    // 把服务端返回的 内容读到 缓存 buf里
    // bytes_read 存长度
    let bytes_read = stream.read(&mut buf)?;
    // 单纯的打印
    println!("[client]: get Message: {}", String::from_utf8_lossy(&buf[..bytes_read]));
    // 关闭 读与写的 双通道
    stream.shutdown(Shutdown::Both).unwrap();

    Ok(())
}