use coyote::{
    CoyoteClient,
    models::{KvGetIn, KvSetIn},
};
use rand::{Rng, SeedableRng, rngs::StdRng};

#[tokio::test]
async fn test_kv_set_get() {
    let client = CoyoteClient::new("xxxxx".to_owned(), None);
    let mut rng = StdRng::seed_from_u64(0);

    let test_key = "test-key".to_string();
    let mut test_val = vec![0u8; 256];
    rng.fill(&mut test_val[..]);

    client
        .kv()
        .set(KvSetIn::new(test_key.clone(), test_val.clone()))
        .await
        .unwrap();

    let resp = client.kv().get(KvGetIn::new(test_key)).await.unwrap();

    assert_eq!(resp.value, test_val);
}
