use super::handlers::index;

#[tokio::test]
async fn test_index_200() {
    let resp = index().await;
    assert_eq!("Hello, web!", resp);
}
