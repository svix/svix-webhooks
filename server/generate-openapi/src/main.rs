use std::io::{self, Write, stdout};

use serde::Serialize as _;

fn main() -> anyhow::Result<()> {
    let mut openapi = svix_server::openapi::initialize_openapi();

    let router = svix_server::v1::docs_router();
    _ = aide::axum::ApiRouter::new()
        .nest("/api/v1", router)
        .finish_api_with(&mut openapi, svix_server::openapi::add_security_scheme);

    // The file checked into git primarily exists to catch accidental changes
    // or divergence between cloud and OSS. No point in having to regenerate it
    // when we bump the server version.
    openapi.info.version = "1.1.1".to_owned();
    openapi.info.description = None;

    svix_server::openapi::postprocess_spec(&mut openapi);
    stdout().write_all(&dump_spec(&openapi)?)?;

    Ok(())
}

struct StripTrailingDotZero<W: Write> {
    inner: W,
    buffer: Vec<u8>,
}

impl<W: Write> StripTrailingDotZero<W> {
    fn new(inner: W) -> Self {
        Self {
            inner,
            buffer: Vec::with_capacity(1024),
        }
    }
}

impl<W: Write> Write for StripTrailingDotZero<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut written = 0;
        for line in buf.split_inclusive(|c| *c == b'\n') {
            written += line.len();
            self.buffer.extend_from_slice(line);
            if !self.buffer.ends_with(b"\n") {
                continue;
            }
            // remove any ".0,\n" at the tail; leaving the ,\n
            if self.buffer.ends_with(b".0,\n") {
                let start = self.buffer.len() - 4;
                self.buffer.drain(start..(start + 2));
            }
            let written = self.inner.write(&self.buffer)?;
            // if https://github.com/rust-lang/rust/issues/140667 existed, we could
            // use a VecDeque and this would be much more elegant
            self.buffer.drain(0..written);
        }
        Ok(written)
    }

    fn flush(&mut self) -> io::Result<()> {
        // remove any ".0,\n" at the tail; leaving the ,\n
        if self.buffer.ends_with(b".0,\n") {
            let start = self.buffer.len() - 4;
            self.buffer.drain(start..(start + 2));
        }
        self.inner.write(&self.buffer)?;
        self.buffer.clear();
        Ok(())
    }
}

fn dump_spec(spec: &aide::openapi::OpenApi) -> anyhow::Result<Vec<u8>> {
    let mut formatted_spec_json = Vec::with_capacity(1024 * 1024);
    let mut writer = StripTrailingDotZero::new(&mut formatted_spec_json);
    let json_formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
    let mut json_serializer = serde_json::Serializer::with_formatter(&mut writer, json_formatter);
    spec.serialize(&mut json_serializer)?;
    writer.flush()?;
    if !formatted_spec_json.ends_with(b"\n") {
        formatted_spec_json.push(b'\n');
    }

    Ok(formatted_spec_json)
}
