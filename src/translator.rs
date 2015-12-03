extern crate xml;
extern crate hyper;
use self::hyper::{Client};
use self::hyper::header::{Headers,Connection};
use self::hyper::client::response::Response;
use self::hyper::status::StatusCode;
use self::xml::reader::{EventReader, XmlEvent};
use std::io::prelude::*;
use std::error::Error;
use stsclient::{StsClient};

pub struct Translator {
    url: &'static str,
    http_client: Client,
    stsclient: StsClient
}

impl Translator {
    pub fn new(
        sts_url: &'static str,
        client_id: &'static str,
        client_secret: &'static str,
        scope: &'static str,
        translator_url: &'static str)
    -> Self {
        Translator {
            url: translator_url,
            http_client: Client::new(),
            stsclient: StsClient::new(sts_url, client_id, client_secret, scope)
        }
    }

    pub fn translate(self, text: String, from: &'static str, to: &'static str) -> String {
        let token: String;
        if let Some(ststoken) = self.stsclient.get_access_token() {
            token = ststoken.access_token;
        } else {
            match self.stsclient.refresh_token() {
                Err(why) => return why,
                Ok(response) => token = response.access_token
            };
        }

        let url_copy = self.url.clone();
        let http_client = &self.http_client;
        let translate_func = || -> Response {
            let auth_token = format!("Bearer {0}", token);
            let requiest_url = format!("{0}?text={1}&from={2}&to={3}", url_copy, text, from, to);

            let mut headers = Headers::new();
            headers.set(Connection::close());
            headers.set_raw("Authorization", vec![auth_token.into_bytes()]);

            http_client
                .get(&*requiest_url)
                .headers(headers)
                .send()
                .unwrap()
        };

        let mut response = translate_func();
        if response.status == StatusCode::Unauthorized {
            match self.stsclient.refresh_token() {
                Err(why) => return why,
                Ok(response) => token = response.access_token
            };
            let mut response = translate_func();
        }

        let mut content = String::new();
        match response.read_to_string(&mut content) {
            Err(why) => Error::description(&why).to_string(),
            Ok(_) => if response.status == StatusCode::Ok {
                self.unwrap_from_xml(&content)
            } else {
                format!("StatusCode: {0}; Response: {1}", response.status, content)
            }
        }
    }

    fn unwrap_from_xml(&self, xml: &str) -> String {
        let parser = EventReader::from_str(xml);
        let chars = parser.into_iter().skip(2).next().unwrap();
        match chars {
            Ok(XmlEvent::Characters(result)) => result.clone(),
            _ => "Failed to parse xml.".to_owned()
        }
    }
}
