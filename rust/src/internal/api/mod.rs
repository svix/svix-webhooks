pub(crate) mod endpoint;
pub(crate) mod endpoint_auto_config;

pub(crate) use endpoint_auto_config::EndpointAutoConfig;

/// Build an internal endpoint client. Lives here (not in `autoconfig`) because codegen keeps
/// `Endpoint::new` as `pub(super)`, visible only inside `internal::api`.
pub(crate) fn client_endpoint<'a>(cfg: &'a crate::Configuration) -> endpoint::Endpoint<'a> {
    endpoint::Endpoint::new(cfg)
}
