#[macro_use]
extern crate rocket;
use solana_client::rpc_client::RpcClient;
use std::env;

#[get("/healthCheck")]
async fn health_check() -> String {
    let solana_endpoint = env::var("SOLANA_ENDPOINT").expect("SOLANA_ENDPOINT not set");
    let rpc: RpcClient = RpcClient::new(solana_endpoint);
    let response = rpc.get_health();

    match response {
        Ok(_) => String::from("Health Ok!"),
        Err(e) => format!("Error: {:?}", e),
    }
}

#[launch]
#[tokio::main]
async fn rocket() -> _ {
    rocket::build().mount("/", routes![health_check])
}
