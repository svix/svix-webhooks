use std::time::Duration;

use svix_server::cfg::load;

// Using a single test in its own integration test binary such that
// `figment::Jail` can't influence other tests.
//
// The only thing it does is save & restore things, it doesn't actually have any
// protections against `jail.set_env` values being seen by other tests -
// internally it calls `std::env::set_var`.
#[test]
fn test_environment_parsing() {
    test_retry_schedule_parsing();
    test_retry_schedule_parsing_legacy();
    test_proxy_addr_from_env_parsing();
}

#[allow(clippy::result_large_err)]
fn test_retry_schedule_parsing() {
    figment::Jail::expect_with(|jail| {
        jail.set_env("SVIX_JWT_SECRET", "x");

        // Multi item
        jail.set_env("SVIX_RETRY_SCHEDULE", "[1,2]");

        let cfg = load().unwrap();
        assert_eq!(
            cfg.retry_schedule,
            vec![Duration::new(1, 0), Duration::new(2, 0)]
        );

        // Single item
        jail.set_env("SVIX_RETRY_SCHEDULE", "[1]");

        let cfg = load().unwrap();
        assert_eq!(cfg.retry_schedule, vec![Duration::new(1, 0)]);

        // Empty
        jail.set_env("SVIX_RETRY_SCHEDULE", "[]");

        let cfg = load().unwrap();
        assert!(cfg.retry_schedule.is_empty());

        Ok(())
    });
}

#[allow(clippy::result_large_err)]
fn test_retry_schedule_parsing_legacy() {
    figment::Jail::expect_with(|jail| {
        jail.set_env("SVIX_JWT_SECRET", "x");

        // Multi item
        jail.set_env("SVIX_RETRY_SCHEDULE", "1,2");

        let cfg = load().unwrap();
        assert_eq!(
            cfg.retry_schedule,
            vec![Duration::new(1, 0), Duration::new(2, 0)]
        );

        // Single item and empty were failing before so not testing them

        Ok(())
    });
}

#[allow(clippy::result_large_err)]
fn test_proxy_addr_from_env_parsing() {
    figment::Jail::expect_with(|jail| {
        jail.set_env("SVIX_QUEUE_TYPE", "memory");
        jail.set_env("SVIX_JWT_SECRET", "x");
        jail.set_env("SVIX_PROXY_ADDR", "socks5://127.0.0.1");

        let cfg = load().unwrap();
        assert!(cfg.proxy_config.is_some());

        Ok(())
    });
}
