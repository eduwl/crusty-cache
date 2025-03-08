use std::sync::OnceLock;

use clap::Parser;

pub static INIT_ARGS: OnceLock<InitArgs> = OnceLock::new();

/// Estrutura de argumentos para inicialização do nó.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct InitArgs {
    /// O modo em que o nó ira operar entre master e slave.
    #[arg(long, default_value = "master")]
    pub mode: String,
    /// IP do servidor mestre para se conectar, sendo slave.
    #[arg(long, default_value = "127.0.0.1")]
    pub master_ip: String,
    /// Porta do servidor mestre para se conectar, sendo slave.
    #[arg(long, default_value = "5555")]
    pub port: u64,
}
