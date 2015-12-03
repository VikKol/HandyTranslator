extern crate hyper;
use self::hyper::{Client};
use self::hyper::header::{Headers,Connection};
use self::hyper::status::StatusCode;

use std::io::prelude::*;
use std::error::Error;
use stsclient::{StsClient};

pub struct Translator {
    url: &'static str,
    stsclient: StsClient,
    http_client: Client
}

impl Translator {
    pub fn new(sts_url: &'static str, client_id: &'static str, client_secret: &'static str, scope: &'static str, translator_url: &'static str) -> Self {
        Translator {
            url: translator_url,
            http_client: Client::new(),
            stsclient: StsClient::new(sts_url, client_id, client_secret, scope)
        }
    }

    pub fn translate(&self, text: String, from: &'static str, to: &'static str) -> String {
        let token: String;
        match self.stsclient.get_access_token() {
            Err(why) => return why,
            Ok(response) => token = response.access_token
        };

        let auth_token = format!("Bearer {0}", token);
        let requiest_url = format!("{0}?text={1}&from={2}&to={3}", self.url, text, from, to);

        let mut headers = Headers::new();
        headers.set(Connection::close());
        headers.set_raw("Authorization", vec![auth_token.into_bytes()]);
        headers.set_raw("Accept", vec!["application/json".to_owned().into_bytes()]);

        let mut response = self.http_client
            .get(&*requiest_url)
            .headers(headers)
            .send()
            .unwrap();

        let mut content = String::new();
        match response.read_to_string(&mut content) {
            Err(why) => Error::description(&why).to_string(),
            Ok(_) => if response.status == StatusCode::Ok {
                content
            } else {
                format!("StatusCode: {0}; Response: {1}", response.status, content)
            }
        }
    }
}
