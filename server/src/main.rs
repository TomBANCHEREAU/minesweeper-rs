use futures_util::{SinkExt, StreamExt};
// use log::*;
use server::grid;
use std::{net::SocketAddr, sync::Arc};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::Error};
use tungstenite::Result;

async fn accept_connection(peer: SocketAddr, stream: TcpStream, state: Arc<()>) {
    let mut ws_stream = accept_async(stream).await.expect("Failed to accept");
    println!("New WebSocket connection: {}", peer);

    while let Some(msg) = ws_stream.next().await {
        // let msg = msg?;
        // if msg.is_text() || msg.is_binary() {
        //     ws_stream.send(msg).await;
        // }
    }
    // if let Err(e) = handle_connection(peer, stream).await {
    //     match e {
    //         Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
    //         err => println!("Error processing connection: {}", err),
    //     }
    // }
}

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:9002";
    let listener = TcpListener::bind(&addr).await.expect("Can't listen");
    println!("Listening on: {}", addr);
    let state = Arc::new(());
    while let Ok((stream, _)) = listener.accept().await {
        let peer = stream
            .peer_addr()
            .expect("connected streams should have a peer address");

        tokio::spawn(accept_connection(peer, stream, Arc::clone(&state)));
    }
}

//

// use std::collections::HashMap;

// fn main() {
//     let mut map: HashMap<i32, i32> = HashMap::new();
//     map.insert(1, 2);
//     map.insert(2, 3);
//     map.insert(3, 4);

//     // Modifier la valeur de l'élément associé à la clé 2 en utilisant la valeur de l'élément associé à la clé 1
//     let mut val2 = map.get_mut(&2).unwrap();
//     *val2 = *map.get(&1).unwrap() + *val2;

//     println!("{:?}", map); // Affiche "{1: 2, 2: 5, 3: 4}"
// }
