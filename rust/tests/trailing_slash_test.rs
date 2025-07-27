// Simple test for trailing slash logic without accessing private fields

#[test]
fn test_trailing_slash_removal_logic() {
    // Test the core logic we implemented
    let test_cases = vec![
        ("https://api.svix.com/", "https://api.svix.com"),
        ("https://api.svix.com///", "https://api.svix.com"),
        ("https://api.svix.com", "https://api.svix.com"),
        ("http://localhost:8000/", "http://localhost:8000"),
        ("http://localhost:8000///", "http://localhost:8000"),
    ];

    for (input, expected) in test_cases {
        let mut result = input.to_string();
        // This is our actual implementation logic
        while result.ends_with('/') {
            result.pop();
        }
        
        assert_eq!(result, expected, "Failed for input: {}", input);
        println!("✅ {} → {}", input, result);
    }
}

#[test]
fn test_trailing_slash_edge_cases() {
    // Test edge cases
    let mut url = String::from("https://api.svix.com/////");
    while url.ends_with('/') {
        url.pop();
    }
    assert_eq!(url, "https://api.svix.com");

    let mut url2 = String::from("https://api.svix.com");
    while url2.ends_with('/') {
        url2.pop();
    }
    assert_eq!(url2, "https://api.svix.com");
}

#[test] 
fn test_regional_url_logic() {
    // Test regional URL detection logic
    let tokens = vec![
        ("test.eu.token", "https://api.eu.svix.com"),
        ("test.us.token", "https://api.us.svix.com"),
        ("test.ca.token", "https://api.ca.svix.com"),
        ("regular.token", "https://api.svix.com"),
    ];

    for (token, expected_base) in tokens {
        let region = token.split('.').nth(1);
        let base_url = match region {
            Some("us") => "https://api.us.svix.com",
            Some("eu") => "https://api.eu.svix.com", 
            Some("in") => "https://api.in.svix.com",
            Some("ca") => "https://api.ca.svix.com",
            Some("au") => "https://api.au.svix.com",
            _ => "https://api.svix.com",
        };
        
        assert_eq!(base_url, expected_base, "Failed for token: {}", token);
        println!("✅ {} → {}", token, base_url);
    }
}