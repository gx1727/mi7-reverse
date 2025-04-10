use tokio::net::TcpStream;
use tokio::io::{copy_bidirectional, AsyncWriteExt};
use tokio::time::{sleep, Duration};

async fn handle_forward(mut server_stream: TcpStream, local_target: &str, product_id: &str) -> anyhow::Result<()> {
    let mut local_stream = TcpStream::connect(local_target).await?;
    println!("[=] 已连接本地服务: {}", local_target);
    println!("[=] 商品ID: {}", product_id);

    // 发送商品ID作为初始数据
    local_stream.write_all(format!("PRODUCT_ID:{}\n", product_id).as_bytes()).await?;

    // 双向复制数据
    copy_bidirectional(&mut server_stream, &mut local_stream).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let server_addr = std::env::var("SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1:7000".to_string());
    let local_target = std::env::var("TARGET_ADDR").unwrap_or_else(|_| "127.0.0.1:9502".to_string());
    let product_id = std::env::var("PRODUCT_ID").expect("必须提供商品ID");

    println!("客户端配置:");
    println!("- 服务器地址: {}", server_addr);
    println!("- 目标地址: {}", local_target);
    println!("- 商品ID: {}", product_id);

    loop {
        println!("[~] 尝试连接到服务器 {}", server_addr);
        match TcpStream::connect(&server_addr).await {
            Ok(stream) => {
                println!("[+] 已连接服务器，开始转发");
                if let Err(e) = handle_forward(stream, &local_target, &product_id).await {
                    eprintln!("[!] 转发错误: {}", e);
                }
            }
            Err(e) => {
                eprintln!("[-] 连接服务器失败: {}，重试中...", e);
            }
        }

        sleep(Duration::from_secs(5)).await;
    }
}
