# A Media Software Development Kit for use on Fastly's Compute@Edge

This crate bundles FFMPEG's libraries, precompiled for WebAssembly. No native installation required.

Compatible with Fastly's Compute@Edge.

## Usage

```toml
[dependencies]
media_sdk = "0"
```

```rust
use fastly::http::{header, Request};
use fastly::{Error, Response};
use media_sdk::media::Media;

fn main() -> Result<(), Error> {
    let mut req = Request::from_client();
    let mut media = Media::new_with_body(req.take_body_bytes())?;
    let jpeg = media.get_jpeg()?;
    let mut resp = Response::new();
    resp.set_header("Content-Type", "image/jpeg");
    resp.set_body(jpeg);
    resp.send_to_client();
    Ok(())
}
```
