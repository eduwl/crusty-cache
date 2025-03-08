use std::{env, process};

use clap::Parser;
use dotenvy::from_filename;
use replication::{INIT_ARGS, InitArgs};
use tokio::{signal, sync::mpsc, task::JoinHandle};

mod memory;
mod replication;
mod socket;

#[tokio::main]
async fn main() {
    {
        // Dotenv load
        from_filename(".env.dev").ok();
        init_node();
    }

    let socket_service_thread = start_socket_service().await;

    manage_shutdown_signals(socket_service_thread).await;
}

fn init_node() {
    let args = InitArgs::parse();
    if INIT_ARGS.set(args).is_err() {
        eprintln!("Falhou ao inicializar os argumentos do nó");
        process::exit(1);
    }
}

async fn start_socket_service() -> tokio::task::JoinHandle<()> {
    tokio::spawn(async {
        for _ in 0..100 {
            println!("{:?}", INIT_ARGS.get().unwrap());
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    })
}

async fn manage_shutdown_signals(socket_service: JoinHandle<()>) {
    let (tx, mut rx) = mpsc::channel::<()>(1);
    let shutdown_signal = tokio::spawn(async move { shutdown(tx).await });

    tokio::select! {
        _ = socket_service => {
            eprintln!("Tarefa do serviço de socket foi concluido");
            process::exit(0);
        },
        _ = shutdown_signal => {
            eprintln!("Recebido sinal de parada...");
            process::exit(0);
        },
        _ = rx.recv() => {
            eprintln!("Recebido sinal de parada...");
            process::exit(0);
        },
    }
}

async fn shutdown(tx: mpsc::Sender<()>) {
    if let Err(e) = signal::ctrl_c().await {
        eprintln!("Erro ao capturar o sinal de shutdown: {}", e);
        process::exit(1);
    }

    if tx.send(()).await.is_err() {
        eprintln!("Erro ao enviar o sinal de shutdown");
    }
}
