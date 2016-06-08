Rust Apple Push Http2 Client Lib
==========


Apple Push client based on http2 new API for rust ;)
Very basic implementation but it seems to work... Cargo not published yet.


## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
apnshttp2 = "0.1.0"
```

and this to your crate root:

```rust
extern crate apnshttp2;
```

on OSX you might need to set OpenSSL path for compilation/linking stage. If you're using brew openssl with bash/zsh etc:

```bash
export OPENSSL_INCLUDE_DIR=`brew --prefix openssl`/include
export OPENSSL_LIB_DIR=`brew --prefix openssl`/lib
```

or if you're using fish:

```fish
set -x OPENSSL_INCLUDE_DIR (brew --prefix openssl)/include
set -x OPENSSL_LIB_DIR (brew --prefix openssl)/lib
```  

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
