use std::collections::VecDeque;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use tokio::io::{copy_bidirectional, AsyncWriteExt};
use tokio::spawn;

type SharedPool = Arc<Mutex<VecDeque<TcpStream>>>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client_addr = std::env::var("CLIENT_ADDR").unwrap_or_else(|_| "0.0.0.0:7000".to_string());
    let user_addr = std::env::var("USER_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".to_string());
    
    let client_listener = TcpListener::bind(&client_addr).await?;
    let user_listener = TcpListener::bind(&user_addr).await?;
    let client_pool: SharedPool = Arc::new(Mutex::new(VecDeque::new()));

    println!("服务器启动在:");
    println!("- 客户端连接地址: {}", client_addr);
    println!("- 用户访问地址: {}", user_addr);

    // 接收 client 保持连接
    {
        let pool = client_pool.clone();
        tokio::spawn(async move {
            loop {
                match client_listener.accept().await {
                    Ok((stream, addr)) => {
                        println!("[+] Client 来自 {} 接入", addr);
                        pool.lock().await.push_back(stream);
                    }
                    Err(e) => eprintln!("Client 接入失败: {}", e),
                }
            }
        });
    }

    // 接收用户访问，转发到客户端
    loop {
        let (mut user_stream, user_addr) = user_listener.accept().await?;
        println!("[*] 用户连接来自: {}", user_addr);

        let pool = client_pool.clone();

        spawn(async move {
            let client_stream_opt = {
                let mut pool = pool.lock().await;
                pool.pop_front()
            };

            match client_stream_opt {
                Some(mut client_stream) => {
                    println!("[=] 开始转发用户 <-> 客户端");
                    if let Err(e) = copy_bidirectional(&mut user_stream, &mut client_stream).await {
                        eprintln!("[!] 转发出错: {}", e);
                    }
                }
                None => {
                    println!("[!] 没有可用客户端");
                    let _ = user_stream.shutdown().await;
                }
            }
        });
    }
}
