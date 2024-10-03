#[tokio::test]
async fn get_existed_order() -> Result<(), httpc_test::Error> {
    let uid = "b563feb7b2b84b6test".to_string();
    let address = "http://localhost:8000";

    let pc = httpc_test::new_client(address)?;
    pc.do_get(format!("/api/get-order?uid={}", &uid).as_str()).await?.print().await?;

    Ok(())
}