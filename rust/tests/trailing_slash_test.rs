use svix::api::{Svix, SvixOptions};

#[test]
fn test_trailing_slash_removal() {
    let svix = Svix::new(
        "key".to_string(),
        Some(SvixOptions {
            server_url: Some("http://localhost:8000/".to_string()),
            ..Default::default()
        }),
    );
    assert_eq!(svix.cfg.base_path, "http://localhost:8000");
}

#[test] 
fn test_no_trailing_slash() {
    let svix = Svix::new(
        "key".to_string(),
        Some(SvixOptions {
            server_url: Some("http://localhost:8000".to_string()),
            ..Default::default()
        }),
    );
    assert_eq!(svix.cfg.base_path, "http://localhost:8000");
}

#[test]
fn test_multiple_trailing_slashes() {
    let svix = Svix::new(
        "key".to_string(),
        Some(SvixOptions {
            server_url: Some("http://localhost:8000///".to_string()),
            ..Default::default()
        }),
    );
    assert_eq!(svix.cfg.base_path, "http://localhost:8000");
}

#[test]
fn test_regional_url_handling() {
    // Test EU region token
    let svix_eu = Svix::new("test.eu.token".to_string(), None);
    assert_eq!(svix_eu.cfg.base_path, "https://api.eu.svix.com");
    
    // Test US region token
    let svix_us = Svix::new("test.us.token".to_string(), None);
    assert_eq!(svix_us.cfg.base_path, "https://api.us.svix.com");
}

#[test]
fn test_default_url() {
    let svix = Svix::new("key".to_string(), None);
    assert_eq!(svix.cfg.base_path, "https://api.svix.com");
}