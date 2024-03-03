use actix_web::{web, App, HttpResponse, HttpServer};
use blockchain_basic::blockchain::{chain::BlockChain, transaction::Transaction};

async fn get_blocks() -> HttpResponse {
    let blockchain = BlockChain::new();
    HttpResponse::Ok().json(&blockchain.chain)
}

async fn mine_block() -> HttpResponse {
    let mut blockchain = BlockChain::new();

    // Add a transaction to the block
    let transaction1 = Transaction::new("Alice".to_string(), "Bob".to_string(), 50.0);
    let transaction2 = Transaction::new("Bob".to_string(), "Charlie".to_string(), 50.0);

    let transactions = vec![transaction1, transaction2];
    blockchain.mine(transactions);

    let mined_block = blockchain.chain.last().unwrap();
    HttpResponse::Ok().json(mined_block)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/blocks", web::post().to(get_blocks))
            .route("/mine", web::get().to(mine_block))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
