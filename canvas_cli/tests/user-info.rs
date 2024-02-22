use canvas_api::create_client;
use canvas_auth::connect;

#[tokio::test]
async fn test_user_info() {
    let auth_token = connect().await.unwrap();
    let client = create_client(auth_token.secret()).unwrap();

    let profile = canvas_api::requests::get_self(client).await;
    assert!(profile.is_ok());
}
