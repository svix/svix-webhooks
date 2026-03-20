use coyote_client::{
    CoyoteClient,
    models::{KvGetIn, KvSetIn},
};
use rand::{Rng, SeedableRng, rngs::StdRng};

#[tokio::test]
#[ignore] // as written, requires the server to be running
async fn test_kv_set_get() {
    let client = CoyoteClient::new("xxxxx".to_owned(), None);
    let mut rng = StdRng::seed_from_u64(0);

    let test_key = "test-key".to_string();
    let mut test_val = vec![0u8; 256];
    rng.fill(&mut test_val[..]);

    client
        .kv()
        .set(test_key.clone(), KvSetIn::new(test_val.clone()))
        .await
        .unwrap();

    let resp = client.kv().get(test_key, KvGetIn::new()).await.unwrap();

    assert_eq!(resp.value, Some(test_val));
}
