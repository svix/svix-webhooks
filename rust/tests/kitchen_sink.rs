use std::collections::HashSet;
use svix::{
    api::{ApplicationIn, EndpointIn, EndpointPatch, EventTypeIn, Svix, SvixOptions},
    error::Error,
};

fn get_test_client() -> Svix {
    let token = std::env::var("SVIX_TOKEN").expect("SVIX_TOKEN is required to run this test");
    let server_url =
        std::env::var("SVIX_SERVER_URL").expect("SVIX_SERVER_URL is required to run this test");
    Svix::new(
        token,
        Some(SvixOptions {
            server_url: Some(server_url),
            ..Default::default()
        }),
    )
}

fn check_for_conflict(e: Error) {
    match e {
        Error::Http(e) => {
            assert_eq!(
                e.status,
                http02::StatusCode::CONFLICT,
                "conflicts are expected but other statuses are not"
            );
        }
        _ => panic!("unexpected error: {e}"),
    }
}

// Opt-in with `cargo test --ignored`
#[ignore]
#[tokio::test]
async fn test_endpoint_crud() {
    let client = get_test_client();

    let app = client
        .application()
        .create(
            ApplicationIn {
                name: "app".to_string(),
                ..Default::default()
            },
            None,
        )
        .await
        .unwrap();

    if let Err(e) = client
        .event_type()
        .create(
            EventTypeIn {
                name: String::from("event.started"),
                description: String::from("Something started"),
                ..Default::default()
            },
            None,
        )
        .await
    {
        check_for_conflict(e);
    }

    if let Err(e) = client
        .event_type()
        .create(
            EventTypeIn {
                name: String::from("event.ended"),
                description: String::from("Something ended"),
                ..Default::default()
            },
            None,
        )
        .await
    {
        check_for_conflict(e);
    }

    let ep = client
        .endpoint()
        .create(
            app.id.clone(),
            EndpointIn {
                channels: Some(vec![String::from("ch0"), String::from("ch1")]),
                url: String::from("https://example.svix.com/"),
                ..Default::default()
            },
            None,
        )
        .await
        .unwrap();

    let want_channels: HashSet<_> = [String::from("ch0"), String::from("ch1")]
        .into_iter()
        .collect();
    let got_channels = ep.channels.clone().unwrap().into_iter().collect();
    assert_eq!(want_channels, got_channels);
    assert_eq!(0, ep.filter_types.unwrap_or_default().len());

    let ep_patched = client
        .endpoint()
        .patch(
            app.id.clone(),
            ep.id.clone(),
            EndpointPatch {
                filter_types: Some(Some(vec![
                    String::from("event.started"),
                    String::from("event.ended"),
                ])),
                ..Default::default()
            },
        )
        .await
        .unwrap();

    let want_filter_types: HashSet<_> =
        [String::from("event.started"), String::from("event.ended")]
            .into_iter()
            .collect();
    let got_channels = ep_patched.channels.clone().unwrap().into_iter().collect();
    let got_filter_types = ep_patched
        .filter_types
        .clone()
        .unwrap()
        .into_iter()
        .collect();
    assert_eq!(want_channels, got_channels);
    assert_eq!(want_filter_types, got_filter_types);

    // Should complete without error if the deserialization handles empty bodies
    // correctly.
    client.endpoint().delete(app.id, ep.id).await.unwrap();
}
