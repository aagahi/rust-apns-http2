
extern crate solicit;
extern crate openssl;

use std::net::TcpStream;


use solicit::client::SimpleClient;
use solicit::http::{HttpScheme, Header, HttpError};
use openssl::ssl::SslMethod::Tlsv1_2;
use openssl::x509::X509;
use openssl::ssl::SSL_OP_NO_COMPRESSION;
use openssl::crypto::pkey::PKey;
use openssl::ssl::{Ssl, SslStream, SslContext};

use solicit::http::ALPN_PROTOCOLS;
use std::str;
use std::io::BufReader;
use std::fs::File;

pub struct APNS {
    ssl_context: SslContext,
    bundle_id: String,
    gateway: String,
}

impl APNS {
    pub fn new(cert_pem_path: &str,
               key_pem_path: &str,
               gateway: &str,
               app_bundle_id: &str)
               -> APNS {
        let mut ctx = SslContext::new(Tlsv1_2).unwrap();

        let cert_reader = &mut BufReader::new(File::open(cert_pem_path).unwrap());
        let x509 = X509::from_pem(cert_reader).unwrap();
        let _ = ctx.set_certificate(&x509);

        let pkey_reader = &mut BufReader::new(File::open(key_pem_path).unwrap());
        let pkey = PKey::private_rsa_key_from_pem(pkey_reader).unwrap();
        let _ = ctx.set_private_key(&pkey);


        ctx.set_options(SSL_OP_NO_COMPRESSION);
        ctx.set_alpn_protocols(ALPN_PROTOCOLS);
        ctx.set_npn_protocols(ALPN_PROTOCOLS);

        APNS {
            ssl_context: ctx,
            bundle_id: app_bundle_id.to_owned(),
            gateway: gateway.to_owned(),
        }
    }


    pub fn new_client(&self) -> Result<SimpleClient<SslStream<TcpStream>>, HttpError> {
        let ssl = Ssl::new(&self.ssl_context).unwrap();
        // ssl.set_hostname(gateway);

        let raw_tcp = TcpStream::connect((self.gateway.as_str(), 443)).unwrap();
        let mut ssl_stream = SslStream::connect(ssl, raw_tcp).unwrap();


        solicit::http::client::write_preface(&mut ssl_stream).unwrap();

        SimpleClient::with_stream(ssl_stream, self.gateway.clone(), HttpScheme::Https)
    }


    pub fn push_client(&self,
                       client: &mut SimpleClient<SslStream<TcpStream>>,
                       device_token: &str,
                       json_str: &str)
                       -> Result<(), HttpError> {


        let json_header = Header::new(b"content-type".to_vec(), b"application/json".to_vec());
        let topic_header = Header::new(b"apns-topic".to_vec(), self.bundle_id.as_bytes());
        let headers = vec![json_header, topic_header];

        let path = format!("/3/device/{}", device_token);

        match client.post(&path.into_bytes(),
                          &headers,
                          json_str.to_string().into_bytes()) {
            Ok(response) => {
                println!("Response code: {}", response.status_code().unwrap());
                println!("{}", str::from_utf8(&response.body).unwrap());
                Ok(())
            }
            Err(err) => Err(err),
        }

        // let response = post_resp.unwrap().recv().unwrap();

        // assert_eq!(response.stream_id, 1);
        // assert_eq!(response.status_code().unwrap(), 200);
        //
        //
        // for header in response.headers.iter() {
        //     println!("{}: {}",
        //              str::from_utf8(header.name()).unwrap(),
        //              str::from_utf8(header.value()).unwrap());
        // }
        // println!("{}", str::from_utf8(&response.body).unwrap());
    }

    pub fn push(&self, device_token: &str, json_str: &str) -> Result<(), HttpError> {
        let mut client = try!(self.new_client());
        try!(self.push_client(&mut client, device_token, json_str));
        Ok(())
    }
}



#[cfg(test)]
mod tests {
    #[test]
    #[warn(unused_must_use)]
    fn push_message() {
        let apns = ::APNS::new("./push-cert.pem",
                               "./push-key.pem",
                               "api.push.apple.com",
                               "bundle_id");

        let mut client = apns.new_client().unwrap();


        let json_str = format!("{{\"aps\":{{\"alert\":\"{}\",\"badge\":1,\"sound\":\
                                \"bingbong.aiff\"}}}}",
                               "Howdy!");
        println!("Push JSON: {}", json_str);

        apns.push_client(&mut client, "token_a", &json_str);
        apns.push_client(&mut client, "token_b", &json_str);

    }
}
