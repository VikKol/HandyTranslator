extern crate xml;
extern crate hyper;
use self::hyper::{Client};
use self::hyper::header::{Headers,Connection};
use self::hyper::client::response::Response;
use self::hyper::status::StatusCode;
use self::xml::reader::{EventReader, XmlEvent};
use std::io::prelude::*;
use stsclient::{StsClient};

pub struct Translator {
    url: &'static str,
    http_client: Client,
    stsclient: StsClient
}

pub type TranslatorResponse = Result<String, String>;

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

    pub fn translate<'a>(&self, text: &'a str, from: &'static str, to: &'static str) -> TranslatorResponse {
        let mut token = match self.stsclient.get_access_token() {
            Some(ststoken) => ststoken,
            None => try!(self.stsclient.refresh_token())
        };

        let mut response = try!(self.translate_request(&token.access_token, &text, &from, &to));
        if response.status == StatusCode::BadRequest {
            token = try!(self.stsclient.refresh_token());
            response = try!(self.translate_request(&token.access_token, &text, &from, &to));
        }

        let mut content = String::new();
        try!(response.read_to_string(&mut content).map_err(|err| format!("{:?}", err)));
        if response.status == StatusCode::Ok {
            self.unwrap_from_xml(&content)
        } else {
            Err(format!("StatusCode: {0}; Response: {1}", response.status, content))
        }
    }

    fn translate_request(&self, token: &str, text: &str, from: &'static str, to: &'static str) -> Result<Response, String> {
        let auth_token = format!("Bearer {0}", token);
        let requiest_url = format!("{0}?text={1}&from={2}&to={3}", self.url, text, from, to);
        let mut headers = Headers::new();
        headers.set(Connection::close());
        headers.set_raw("Authorization", vec![auth_token.into_bytes()]);

        self.http_client
            .get(&*requiest_url)
            .headers(headers)
            .send()
            .map_err(|err| format!("translate_request error: {:?}", err))
    }

    fn unwrap_from_xml(&self, xml: &str) -> TranslatorResponse {
        let parser = EventReader::from_str(xml);
        let chars = parser.into_iter().skip(2).next().unwrap();
        if let Ok(XmlEvent::Characters(result)) = chars {
            return Ok(result.clone());
        }
        Err("Failed to parse xml.".to_owned())
    }
}
