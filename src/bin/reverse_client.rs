use tokio::net::TcpStream;
use tokio::io::copy_bidirectional;
use tokio::time::{sleep, Duration};

async fn handle_forward(mut server_stream: TcpStream, local_target: &str) -> anyhow::Result<()> {
    let mut local_stream = TcpStream::connect(local_target).await?;
    println!("[=] 已连接本地服务: {}", local_target);

    // 双向复制数据
    copy_bidirectional(&mut server_stream, &mut local_stream).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let server_addr = "127.0.0.1:7000"; // 改成你的服务器地址
    let local_target = "127.0.0.1:9502";     // 改成你的内网目标服务

    loop {
        println!("[~] 尝试连接到服务器 {}", server_addr);
        match TcpStream::connect(server_addr).await {
            Ok(stream) => {
                println!("[+] 已连接服务器，开始转发");
                if let Err(e) = handle_forward(stream, local_target).await {
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
