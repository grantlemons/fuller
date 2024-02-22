use canvas_api::*;
use canvas_auth::connect;

#[tokio::main]
async fn main() {
    let auth_token = connect().await.unwrap();
}
