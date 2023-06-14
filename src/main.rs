use axum::{response::Html, routing::get, Router};
use bdk::{
    bitcoin::Network, blockchain::ElectrumBlockchain, database::MemoryDatabase,
    electrum_client::Client, wallet::AddressIndex, SyncOptions, Wallet,
};
use dotenv::from_filename;
use std::{env, ffi::OsStr, net::SocketAddr, path::Path};

#[derive(serde::Serialize)]
struct AddressResponse {
    address: String,
    index: u32,
}

#[tokio::main]
async fn main() {
    from_filename(".env").ok();

    let descriptor = env::var(OsStr::new("WALLET_DESCRIPTOR")).unwrap();

    let wallet = Wallet::new(
        &descriptor,
        None,
        Network::Testnet,
        MemoryDatabase::default(), // SqliteDatabase::new(Path::new("test1.db")), //
    )
    .expect("error wallet init");

    // Axum
    let app = Router::new().route("/", get(handler));

    // run it
    // let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
    //     .await
    //     .unwrap();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    // Axum

    let blockchain = ElectrumBlockchain::from(
        Client::new("ssl://electrum.blockstream.info:60002").expect("block error"),
    );
    wallet
        .sync(&blockchain, SyncOptions::default())
        .expect("sync error");

    let balance = wallet.get_balance();

    let address = wallet.get_address(AddressIndex::New);
}

async fn handler() -> Html<&'static str> {
    // let resp = AddressResponse {
    //     address: "test".to_string(),
    //     index: 0,
    // };
    Html("<h1>Hello, World!</h1>")
}

// async fn handler() -> Result<impl IntoResponse, AppError> {
//     let resp = AddressResponse {
//         address: "test".to_string(),
//         index: 0,
//     };
//     Ok(Json(resp));
// }
// ref: https://github.com/futurepaul/paypaul/blob/frontend/src/main.rs
