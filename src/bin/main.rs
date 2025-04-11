use clap::{Parser, Subcommand};
use mi7_reverse::{server, client, config::Config};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// 配置文件路径
    #[arg(short, long, default_value = "config.toml")]
    config: PathBuf,
}

#[derive(Subcommand)]
enum Commands {
    /// 启动服务器
    Server,
    /// 启动客户端
    Client {
        /// 商品ID
        #[arg(short, long)]
        product_id: String,
    },
    /// 同时启动服务器和客户端
    Start {
        /// 商品ID
        #[arg(short, long)]
        product_id: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let config = Config::load(&cli.config)?;

    match cli.command {
        Commands::Server => {
            println!("正在启动服务器...");
            server::run_server(&config.server.client_addr, &config.server.user_addr).await?;
        }
        Commands::Client { product_id } => {
            println!("正在启动客户端...");
            client::run_client(&config.client.server_addr, &config.client.target_addr, &product_id).await?;
        }
        Commands::Start { product_id } => {
            println!("正在同时启动服务器和客户端...");
            let server_config = config.server.clone();
            let client_config = config.client.clone();
            
            let server_handle = tokio::spawn(async move {
                server::run_server(&server_config.client_addr, &server_config.user_addr).await
            });
            
            let client_handle = tokio::spawn(async move {
                client::run_client(&client_config.server_addr, &client_config.target_addr, &product_id).await
            });
            
            tokio::select! {
                result = server_handle => result??,
                result = client_handle => result??,
            }
        }
    }

    Ok(())
} 