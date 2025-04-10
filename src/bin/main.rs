use clap::{Parser, Subcommand};
use std::process;
use tokio::process::Command;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 启动服务器
    Server {
        /// 服务器监听地址
        #[arg(short, long, default_value = "0.0.0.0:7000")]
        client_addr: String,
        
        /// 用户访问地址
        #[arg(short, long, default_value = "0.0.0.0:8080")]
        user_addr: String,
    },
    
    /// 启动客户端
    Client {
        /// 服务器地址
        #[arg(short, long, default_value = "127.0.0.1:7000")]
        server_addr: String,
        
        /// 本地目标服务地址
        #[arg(short, long, default_value = "127.0.0.1:9502")]
        target_addr: String,
        
        /// 商品ID
        #[arg(short, long)]
        product_id: String,
    },
    
    /// 同时启动服务器和客户端
    Start {
        /// 服务器监听地址
        #[arg(short, long, default_value = "0.0.0.0:7000")]
        client_addr: String,
        
        /// 用户访问地址
        #[arg(short, long, default_value = "0.0.0.0:8080")]
        user_addr: String,
        
        /// 本地目标服务地址
        #[arg(short, long, default_value = "127.0.0.1:9502")]
        target_addr: String,
        
        /// 商品ID
        #[arg(short, long)]
        product_id: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Server { client_addr, user_addr } => {
            println!("启动服务器...");
            println!("客户端连接地址: {}", client_addr);
            println!("用户访问地址: {}", user_addr);
            
            let status = Command::new("cargo")
                .args(["run", "--bin", "reverse_server"])
                .env("CLIENT_ADDR", client_addr)
                .env("USER_ADDR", user_addr)
                .status()
                .await?;
                
            if !status.success() {
                process::exit(1);
            }
        }
        
        Commands::Client { server_addr, target_addr, product_id } => {
            println!("启动客户端...");
            println!("服务器地址: {}", server_addr);
            println!("目标地址: {}", target_addr);
            println!("商品ID: {}", product_id);
            
            let status = Command::new("cargo")
                .args(["run", "--bin", "reverse_client"])
                .env("SERVER_ADDR", server_addr)
                .env("TARGET_ADDR", target_addr)
                .env("PRODUCT_ID", product_id)
                .status()
                .await?;
                
            if !status.success() {
                process::exit(1);
            }
        }
        
        Commands::Start { client_addr, user_addr, target_addr, product_id } => {
            println!("同时启动服务器和客户端...");
            
            let client_addr_clone = client_addr.clone();
            let target_addr_clone = target_addr.clone();
            let product_id_clone = product_id.clone();
            
            // 启动服务器
            let server_handle = tokio::spawn(async move {
                Command::new("cargo")
                    .args(["run", "--bin", "reverse_server"])
                    .env("CLIENT_ADDR", client_addr)
                    .env("USER_ADDR", user_addr)
                    .status()
                    .await
            });
            
            // 等待服务器启动
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
            
            // 启动客户端
            let client_handle = tokio::spawn(async move {
                Command::new("cargo")
                    .args(["run", "--bin", "reverse_client"])
                    .env("SERVER_ADDR", format!("127.0.0.1:{}", client_addr_clone.split(':').last().unwrap_or("7000")))
                    .env("TARGET_ADDR", target_addr_clone)
                    .env("PRODUCT_ID", product_id_clone)
                    .status()
                    .await
            });
            
            // 等待两个进程完成
            let (server_result, client_result) = tokio::join!(server_handle, client_handle);
            
            if let Err(e) = server_result? {
                eprintln!("服务器错误: {}", e);
                process::exit(1);
            }
            
            if let Err(e) = client_result? {
                eprintln!("客户端错误: {}", e);
                process::exit(1);
            }
        }
    }

    Ok(())
} 