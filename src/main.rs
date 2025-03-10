use std::{process, sync::Arc};

use clap::Parser;
use dotenvy::from_filename;
use replication::{INIT_ARGS, InitArgs, Replica};
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

    let node = match replication::create_node() {
        Ok(n) => n,
        Err(e) => {
            eprintln!("Falha ao criar o nó: {}", e);
            process::exit(1);
        }
    };
    let replica = Arc::new(Replica::new(node));

    let socket_replication_thread = start_replication_thread(replica.clone()).await;
    let socket_service_thread = start_socket_service(replica.clone()).await;

    manage_shutdown_signals(socket_replication_thread, socket_service_thread).await;
}

fn init_node() {
    let args = InitArgs::parse();
    if INIT_ARGS.set(args).is_err() {
        eprintln!("Falhou ao inicializar os argumentos do nó");
        process::exit(1);
    }
}

async fn start_replication_thread(replica: Arc<Replica>) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async {
        if let Err(e) = replication::start(replica).await {
            eprintln!("Falha ao iniciar o serviço de replicação: {}", e);
            process::exit(1);
        }
    })
}

async fn start_socket_service(replica: Arc<Replica>) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async {
        if let Err(e) = socket::start(replica).await {
            eprintln!("Falha ao iniciar o serviço de socket: {}", e);
            process::exit(1);
        }
    })
}

async fn manage_shutdown_signals(
    socket_replication: JoinHandle<()>,
    socket_service: JoinHandle<()>,
) {
    let (tx, mut rx) = mpsc::channel::<()>(1);
    let shutdown_signal = tokio::spawn(async move { shutdown(tx).await });

    tokio::select! {
        _ = socket_replication => {
            eprintln!("Tarefa do serviço de replicação foi concluido");
            process::exit(0);
        },
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
