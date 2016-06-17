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

## Setup


We use OpenSSL 1.0.2 so you might need to perform some update to your operating system.

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

### Generate cert/key file

I haven't figured out how to directly use pk12 file so you need to generate cert/pem file:

```bash
openssl pkcs12 -in push.p12 -clcerts -out push-cert.pem
openssl pkcs12 -in push.p12 -nocerts -nodes | openssl rsa > push-key.pem
```

### Usage

You need to create a apns to retrive a https/ssl client and then you can use the client connection for each message sent to specific token.

```rust
let apns = APNS::new("./push-cert.pem", "./push-key.pem", "api.push.apple.com", "itune_bundle_id");

let mut client = apns.new_client().unwrap();
let json_str = format!("{{\"aps\":{{\"alert\":\"{}\",\"badge\":1,\"sound\":\
                      \"bingbong.aiff\"}}}}",
                       "Howdy!");
apns.push_client(&mut client, "token_a", &json_str);
apns.push_client(&mut client, "token_b", &json_str);
```

You can also check Apple Push documentation
for [payload format](https://developer.apple.com/library/ios/documentation/NetworkingInternet/Conceptual/RemoteNotificationsPG/Chapters/TheNotificationPayload.html#//apple_ref/doc/uid/TP40008194-CH107-SW1)
and [protocol](https://developer.apple.com/library/ios/documentation/NetworkingInternet/Conceptual/RemoteNotificationsPG/Chapters/APNsProviderAPI.html#//apple_ref/doc/uid/TP40008194-CH101-SW15).

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
