use std::time::{Duration, Instant};

use client::{ls, mkdir, Conn, ReadServer, ServerList, WriteServer};

fn serverlist_from_args() -> ServerList {
    let mut args = std::env::args();
    let port = args.nth(1).unwrap().parse().unwrap();
    ServerList {
        port,
        write_serv: None,
        read_serv: None,
        fallback: args.map(|a| a.parse().unwrap()).collect(),
    }
}

fn setup_tracing() {
    use tracing_subscriber::FmtSubscriber;
    let _subscriber = FmtSubscriber::builder().try_init().unwrap();
}

async fn make_list(wconn: &mut WriteServer, prefix: &str) {
    for numb in 0..20 {
        let path = format!("{}/{}", prefix, numb);
        mkdir(wconn, &path).await;
    }
}

#[tokio::main]
async fn main() {
    println!("bench ls started");
    setup_tracing();
    let list = serverlist_from_args();

    let mut wconn = WriteServer::from_serverlist(list.clone()).await.unwrap();

    use rand::{distributions::Alphanumeric, Rng};
    let prefix: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();
    make_list(&mut wconn, &prefix).await;
    tracing::info!("made list");

    let start = Instant::now();
    let tasks = (0..2000).into_iter().map(|_| {
        let list = list.clone();
        let prefix = prefix.clone();
        tokio::spawn(async move {
            let mut rconn = ReadServer::from_serverlist(list).await.unwrap();
            for _ in 0..1000 {
                let _ = ls(&mut rconn, &prefix).await;
            }
        })
    });
    futures::future::join_all(tasks).await;
    println!("elapsed: {:?}", start.elapsed());
}
