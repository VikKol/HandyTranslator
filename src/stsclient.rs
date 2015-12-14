extern crate url;
extern crate serde_json;
extern crate hyper;
use self::url::percent_encoding::{FORM_URLENCODED_ENCODE_SET,percent_encode};
use self::hyper::{Client};
use self::hyper::status::StatusCode;

use std::io::prelude::*;
use std::collections::BTreeMap;
use std::cell::RefCell;

pub struct StsClient {
    base_url: &'static str,
    request_details: String,
    http_client: Client,
    token: RefCell<Option<StsToken>>
}
unsafe impl Sync for StsClient {}

#[derive(Clone)]
pub struct StsToken {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: String,
    pub scope: String
}

pub type StsResponse = Result<StsToken, String>;

impl StsClient {
    pub fn new(base_url: &'static str, client_id: &'static str, client_secret: &'static str, scope: &'static str) -> Self {
        StsClient {
            base_url: base_url,
            request_details: format!("grant_type=client_credentials&client_id={0}&client_secret={1}&scope={2}",
                                      percent_encode(client_id.to_string().as_bytes(), FORM_URLENCODED_ENCODE_SET),
                                      percent_encode(client_secret.to_string().as_bytes(), FORM_URLENCODED_ENCODE_SET),
                                      scope),
            http_client: Client::new(),
            token: RefCell::new(None)
        }
    }

    pub fn get_access_token(&self) -> Option<StsToken> {
        self.token.borrow().clone()
    }

    pub fn refresh_token(&self) -> StsResponse {
        self.http_client
            .post(self.base_url)
            .body(&self.request_details)
            .send()
            .map_err(|err| format!("Failed to send the request: {:?}", err))
            .and_then(|mut response|{
                let mut content = String::new();
                try!(response.read_to_string(&mut content)
                    .map_err(|err| format!("Failed to read the response: {:?}", err)));
                if response.status == StatusCode::Ok {
                    let mut token_mut = self.token.borrow_mut();
                    *token_mut = Some(self.deserialize_content(&content));
                    Ok(token_mut.as_ref().unwrap().clone())
                } else {
                    Err(format!("{:?}", response.status_raw()))
                }
            })
    }

    fn deserialize_content(&self, content: &String) -> StsToken {
        let deserialized_map: BTreeMap<String, String> = serde_json::from_str(&content).unwrap();
        StsToken {
            scope: deserialized_map[&"scope".to_owned()].clone(),
            token_type: deserialized_map[&"token_type".to_owned()].clone(),
            expires_in: deserialized_map[&"expires_in".to_owned()].clone(),
            access_token: deserialized_map[&"access_token".to_owned()].clone()
        }
    }
}
